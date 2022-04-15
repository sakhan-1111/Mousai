use adw::prelude::*;
use gettextrs::gettext;
use gtk::{
    glib::{self, clone},
    subclass::prelude::*,
};
use once_cell::unsync::OnceCell;

use super::audio_visualizer::AudioVisualizer;
use crate::recognizer::{Recognizer, RecognizerState};

mod imp {
    use super::*;
    use gtk::CompositeTemplate;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/seadve/Mousai/ui/recognizer-view.ui")]
    pub struct RecognizerView {
        #[template_child]
        pub(super) title: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) visualizer: TemplateChild<AudioVisualizer>,

        pub(super) recognizing_animation: OnceCell<adw::TimedAnimation>,
        pub(super) recognizer: OnceCell<Recognizer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RecognizerView {
        const NAME: &'static str = "MsaiRecognizerView";
        type Type = super::RecognizerView;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RecognizerView {
        fn dispose(&self, obj: &Self::Type) {
            while let Some(child) = obj.first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for RecognizerView {}
}

glib::wrapper! {
    pub struct RecognizerView(ObjectSubclass<imp::RecognizerView>)
        @extends gtk::Widget;
}

impl RecognizerView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create RecognizerView")
    }

    /// Must be only called once
    pub fn bind_recognizer(&self, recognizer: &Recognizer) {
        recognizer.connect_state_notify(clone!(@weak self as obj => move |_| {
            obj.update_stack();
        }));

        let audio_recorder = recognizer.audio_recorder();
        audio_recorder.connect_peak_notify(clone!(@weak self as obj => move |recorder| {
            let peak = 10_f64.powf(recorder.peak() / 20.0);
            obj.imp().visualizer.push_peak(peak as f64);
        }));
        audio_recorder.connect_stopped(clone!(@weak self as obj => move |_| {
            obj.imp().visualizer.clear_peaks();
        }));

        self.imp().recognizer.set(recognizer.clone()).unwrap();

        self.update_stack();
    }

    fn recognizer(&self) -> &Recognizer {
        self.imp().recognizer.get_or_init(|| {
            log::error!("Recognizer was not bound in RecognizerView. Creating a default one.");
            Recognizer::default()
        })
    }

    fn update_stack(&self) {
        let imp = self.imp();

        match self.recognizer().state() {
            RecognizerState::Listening => {
                if let Some(recognizing_animation) = imp.recognizing_animation.get() {
                    imp.visualizer.clear_peaks();
                    recognizing_animation.pause();
                }

                imp.title.set_label(&gettext("Listening…"));
            }
            RecognizerState::Recognizing => {
                let animation = imp.recognizing_animation.get_or_init(|| {
                    adw::TimedAnimation::builder()
                        .widget(&imp.visualizer.get())
                        .value_from(0.0)
                        .value_to(0.6)
                        .duration(1500)
                        .target(&adw::CallbackAnimationTarget::new(Some(Box::new(
                            clone!(@weak self as obj => move |value| {
                                obj.imp().visualizer.push_peak(value);
                            }),
                        ))))
                        .easing(adw::Easing::EaseOutExpo)
                        .repeat_count(u32::MAX)
                        .alternate(true)
                        .build()
                });
                imp.visualizer.clear_peaks();
                animation.play();

                imp.title.set_label(&gettext("Recognizing…"));
            }
            RecognizerState::Null => {
                if let Some(recognizing_animation) = imp.recognizing_animation.get() {
                    imp.visualizer.clear_peaks();
                    recognizing_animation.pause();
                }
            }
        }
    }
}

impl Default for RecognizerView {
    fn default() -> Self {
        Self::new()
    }
}