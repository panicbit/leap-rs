#include <cstdint>
#include <Leap.h>
#include "hand.h"
#include "vector.h"
#include "list.h"

extern "C" {
    int32_t lm_hand_id(LM_Hand self) {
        return self->id();
    }

    float lm_hand_pinch_distance(LM_Hand self) {
        // TODO: use pinchDistance() once Leap SDK 3.x releases
        return self->pinchStrength();
    }

    LM_Vector lm_hand_stabilized_palm_position(LM_Hand self) {
        return new Leap::Vector(self->stabilizedPalmPosition());
    }

    void lm_hand_delete(LM_Hand self) {
        delete self;
    }

    LM_LIST_IMPL(Hand, hand)
    LM_LIST_SPATIAL_IMPL(Hand, hand)
}