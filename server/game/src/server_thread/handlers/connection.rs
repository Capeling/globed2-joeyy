use std::sync::atomic::Ordering;

use globed_shared::{crypto_box::ChaChaBox, logger::*, PROTOCOL_VERSION};

use crate::server_thread::{GameServerThread, PacketHandlingError};

use super::*;
use crate::data::*;

impl GameServerThread {
    gs_handler!(self, handle_ping, PingPacket, packet, {
        self.send_packet_fast(&PingResponsePacket {
            id: packet.id,
            player_count: self.game_server.state.player_count.load(Ordering::Relaxed),
        })
        .await
    });

    gs_handler!(self, handle_crypto_handshake, CryptoHandshakeStartPacket, packet, {
        match packet.protocol {
            p if p > PROTOCOL_VERSION => {
                gs_disconnect!(
                    self,
                    format!(
                        "Outdated server! You are running protocol v{p} while the server is still on v{PROTOCOL_VERSION}.",
                    )
                    .try_into()?
                );
            }
            p if p < PROTOCOL_VERSION => {
                gs_disconnect!(
                    self,
                    format!(
                        "Outdated client! Please update the mod in order to connect to the server. Client protocol version: v{p}, server: v{PROTOCOL_VERSION}",
                    ).try_into()?
                );
            }
            _ => {}
        }

        {
            // as ServerThread is now tied to the SocketAddrV4 and not account id like in globed v0
            // erroring here is not a concern, even if the user's game crashes without a disconnect packet,
            // they would have a new randomized port when they restart and this would never fail.
            if self.crypto_box.get().is_some() {
                self.disconnect(FastString::from_str(
                    "attempting to perform a second handshake in one session",
                ))
                .await?;
                return Err(PacketHandlingError::WrongCryptoBoxState);
            }

            self.crypto_box
                .get_or_init(|| ChaChaBox::new(&packet.key.0, &self.game_server.secret_key));
        }

        self.send_packet_fast(&CryptoHandshakeResponsePacket {
            key: self.game_server.public_key.clone().into(),
        })
        .await
    });

    gs_handler!(self, handle_keepalive, KeepalivePacket, _packet, {
        let _ = gs_needauth!(self);

        self.send_packet_fast(&KeepaliveResponsePacket {
            player_count: self.game_server.state.player_count.load(Ordering::Relaxed),
        })
        .await
    });

    gs_handler!(self, handle_login, LoginPacket, packet, {
        // disconnect if server is under maintenance
        if self.game_server.central_conf.lock().maintenance {
            gs_disconnect!(
                self,
                FastString::from_str("The server is currently under maintenance, please try connecting again later.")
            );
        }

        // skip authentication if standalone
        let standalone = self.game_server.standalone;
        let player_name = if standalone {
            packet.name
        } else {
            // lets verify the given token
            match self
                .game_server
                .token_issuer
                .validate(packet.account_id, packet.token.to_str().unwrap())
            {
                Ok(x) => FastString::from_str(&x),
                Err(err) => {
                    self.terminate();
                    let mut message = FastString::from_str("authentication failed: ");
                    message.extend(err.error_message()); // no need to use extend_safe as the messages are pretty short
                    self.send_packet_fast(&LoginFailedPacket { message }).await?;

                    return Ok(());
                }
            }
        };

        self.game_server.check_already_logged_in(packet.account_id)?;
        self.account_id.store(packet.account_id, Ordering::Relaxed);
        self.game_server.state.player_count.fetch_add(1u32, Ordering::Relaxed); // increment player count

        info!(
            "Login successful from {player_name} (account ID: {}, address: {})",
            packet.account_id, self.peer
        );

        {
            let mut account_data = self.account_data.lock();
            account_data.account_id = packet.account_id;
            account_data.icons.clone_from(&packet.icons);
            account_data.name = player_name;

            if !standalone {
                let special_user_data = self
                    .game_server
                    .central_conf
                    .lock()
                    .special_users
                    .get(&packet.account_id)
                    .cloned();

                if let Some(sud) = special_user_data {
                    account_data.special_user_data = Some(sud.try_into()?);
                }
            }
        }

        // add them to the global room
        self.game_server
            .state
            .room_manager
            .get_global()
            .create_player(packet.account_id);

        let tps = self.game_server.central_conf.lock().tps;
        self.send_packet_fast(&LoggedInPacket { tps }).await?;

        Ok(())
    });

    gs_handler_sync!(self, handle_disconnect, DisconnectPacket, _packet, {
        self.terminate();
        Ok(())
    });
}
