#include "stream.hpp"

#if GLOBED_VOICE_SUPPORT

#include "manager.hpp"

AudioStream::AudioStream(AudioDecoder&& decoder) : decoder(std::move(decoder)) {
    FMOD_CREATESOUNDEXINFO exinfo = {};

    // TODO figure it out in 2.2. the size is erroneously calculated as 144 on android.
#ifdef GLOBED_ANDROID
    exinfo.cbsize = 140;
#else
    exinfo.cbsize = sizeof(FMOD_CREATESOUNDEXINFO);
#endif

    exinfo.numchannels = 1;
    exinfo.format = FMOD_SOUND_FORMAT_PCMFLOAT;
    exinfo.defaultfrequency = VOICE_TARGET_SAMPLERATE;
    exinfo.userdata = this;
    exinfo.length = sizeof(float) * exinfo.numchannels * exinfo.defaultfrequency * (VOICE_CHUNK_RECORD_TIME * EncodedAudioFrame::VOICE_MAX_FRAMES_IN_AUDIO_FRAME);

    // geode::log::debug("{}: creating stream, length: {}", util::time::nowPretty(), exinfo.length);
    exinfo.pcmreadcallback = [](FMOD_SOUND* sound_, void* data, unsigned int len) -> FMOD_RESULT {
        FMOD::Sound* sound = reinterpret_cast<FMOD::Sound*>(sound_);
        AudioStream* stream = nullptr;
        sound->getUserData((void**)&stream);

        if (!stream) {
            geode::log::debug("audio stream is nullptr in cb, ignoring");
            return FMOD_OK;
        }

        // write data..

        size_t neededSamples = len / sizeof(float);
        size_t copied = stream->queue.copyTo((float*)data, neededSamples);

        if (copied != neededSamples) {
            stream->starving = true;
            // fill the rest with the void to not repeat stuff
            for (size_t i = copied; i < neededSamples; i++) {
                ((float*)data)[i] = 0.0f;
            }
        } else {
            stream->starving = false;
        }

        return FMOD_OK;
    };

    auto& vm = GlobedAudioManager::get();

    FMOD_RESULT res;
    auto system = vm.getSystem();
    res = system->createStream(nullptr, FMOD_OPENUSER | FMOD_2D | FMOD_LOOP_NORMAL, &exinfo, &sound);

    GLOBED_REQUIRE(res == FMOD_OK, GlobedAudioManager::formatFmodError(res, "System::createStream"))
}

AudioStream::~AudioStream() {
    if (channel) {
        channel->stop();
        channel = nullptr;
    }

    if (sound) {
        sound->release();
        sound = nullptr;
    }
}

void AudioStream::start() {
    this->channel = GlobedAudioManager::get().playSound(sound);
}

void AudioStream::writeData(const EncodedAudioFrame& frame) {
    const auto& frames = frame.getFrames();
    for (const auto& opusFrame : frames) {
        auto decodedFrame = decoder.decode(opusFrame);
        queue.writeData(decodedFrame);

        AudioDecoder::freeData(decodedFrame);
    }
}

#endif // GLOBED_VOICE_SUPPORT