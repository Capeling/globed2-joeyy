#pragma once
#include <defs/geode.hpp>

#include <Geode/modify/PlayLayer.hpp>
#include <Geode/modify/GJBaseGameLayer.hpp>

class $modify(GlobedPlayLayer, PlayLayer) {

    static void onModify(auto& self) {
        (void) self.setHookPriority("PlayLayer::resetLevel", 99999999);
    }

    // gd hooks

    $override
    bool init(GJGameLevel* level, bool p1, bool p2);

    $override
    void setupHasCompleted();

    $override
    void onQuit();

    $override
    void fullReset();

    $override
    void resetLevel();

    $override
    void showNewBest(bool p0, int p1, int p2, bool p3, bool p4, bool p5);

    $override
    void levelComplete();

    $override
    void destroyPlayer(PlayerObject* p0, GameObject* p1);
};
