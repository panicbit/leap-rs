use raw;
use PointableList;
use InteractionBox;

pub struct Frame {
    raw: *mut raw::Frame
}

impl Frame {
    pub unsafe fn from_raw(raw: *mut raw::Frame) -> Frame {
        Frame {
            raw: raw
        }
    }

    pub fn current_fps(&self) -> f32 {
        unsafe {
            raw::lm_frame_current_frames_per_second(self.raw)
        }
    }

    pub fn pointables(&self) -> PointableList {
        unsafe {
            PointableList::from_raw(raw::lm_frame_pointables(self.raw))
        }
    }

    pub fn interaction_box(&self) -> InteractionBox {
        unsafe {
            InteractionBox::from_raw(raw::lm_frame_interaction_box(self.raw))
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            raw::lm_frame_delete(self.raw);
        }
    }
}
