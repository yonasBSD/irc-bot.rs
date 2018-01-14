use super::ModuleFeatureInfo;
use super::ModuleInfo;
use irc;
use serde_yaml;
use std::any::Any;
use std::borrow::Cow;
use std::io;
use util;

error_chain! {
    foreign_links {
        Io(io::Error);
        SerdeYaml(serde_yaml::Error);
    }

    links {
        IrcCrate(irc::error::Error, irc::error::ErrorKind);
        YamlUtil(util::yaml::Error, util::yaml::ErrorKind);
    }

    errors {
        ModuleRegistryClash(old: ModuleInfo, new: ModuleInfo) {
            description("module registry clash")
            display("Failed to load a new module because it would have overwritten an old module. \
                     Old: {:?}; new: {:?}.",
                    old,
                    new)
        }

        ModuleFeatureRegistryClash(old: ModuleFeatureInfo, new: ModuleFeatureInfo) {
            description("module feature registry clash")
            display("Failed to load a new module feature because it would have overwritten an old \
                     module feature. Old: {:?}; new: {:?}.",
                    old,
                    new)
        }

        Config(key: String, problem: String) {
            description("configuration error")
            display("Configuration error: Key {:?} {}.", key, problem)
        }

        ThreadSpawnFailure(io_err: io::Error) {
            description("failed to spawn thread")
            display("Failed to spawn thread: {}", io_err)
        }

        HandlerPanic(
            feature_kind: Cow<'static, str>,
            feature_name: Cow<'static, str>,
            payload: Box<Any + Send + 'static>
        ) {
            description("panic in module feature handler function")
            display("The handler function for {} {:?} panicked with the following message: {}",
                    feature_kind,
                    feature_name,
                    util::fmt::FmtAny(payload.as_ref()))
        }

        NicknameUnknown {
            description("nickname retrieval error")
            display("Puzzlingly, the bot seems to have forgotten its own nickname.")
        }

        LockPoisoned(lock_contents_desc: Cow<'static, str>) {
            description("lock poisoned")
            display("A thread panicked, poisoning a lock around {}.", lock_contents_desc)
        }

        Any(inner: Box<Any + Send + 'static>) {
            description("miscellaneous error")
            display("Error: {}", util::fmt::FmtAny(inner.as_ref()))
        }

        Unit {
            description("unknown error")
            display("An error seems to have occurred, but unfortunately the error type provided \
                     was the unit type, containing no information about the error.")
        }

        #[doc(hidden)]
        __Nonexhaustive {
            description("<secret error>")
            display("An error has occurred. In particular, someone used the error kind \
                     `__Nonexhaustive`, which should never be used.")
        }
    }
}
