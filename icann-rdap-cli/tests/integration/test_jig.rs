use assert_cmd::Command;
use icann_rdap_srv::config::ListenConfig;
use icann_rdap_srv::server::AppState;
use icann_rdap_srv::server::Listener;
use icann_rdap_srv::storage::mem::ops::Mem;
use std::time::Duration;
use test_dir::DirBuilder;
use test_dir::FileType;
use test_dir::TestDir;

pub struct TestJig {
    pub mem: Mem,
    pub cmd: Command,
    // pass ownership to the test so the directories are dropped when the test is done.
    _test_dir: TestDir,
}

impl TestJig {
    pub fn new() -> TestJig {
        let mem = Mem::default();
        let app_state = AppState {
            storage: mem.clone(),
        };
        let _ = tracing_subscriber::fmt().try_init();
        let listener = Listener::listen(&ListenConfig::default()).expect("listening on interface");
        let rdap_base = listener.rdap_base();
        tokio::spawn(async move {
            listener
                .start_with_state(app_state)
                .await
                .expect("starting server");
        });
        let test_dir = TestDir::temp()
            .create("cache", FileType::Dir)
            .create("config", FileType::Dir);
        let mut cmd = Command::cargo_bin("rdap").expect("cannot find rdap cmd");
        cmd.env_clear()
            .timeout(Duration::from_secs(2))
            .env("RDAP_BASE_URL", rdap_base)
            .env("RDAP_PAGING", "none")
            .env("RDAP_OUTPUT", "json-extra")
            .env("RDAP_LOG", "debug")
            .env("XDG_CACHE_HOME", test_dir.path("cache"))
            .env("XDG_CONFIG_HOME", test_dir.path("config"));
        TestJig {
            mem,
            cmd,
            _test_dir: test_dir,
        }
    }
}
