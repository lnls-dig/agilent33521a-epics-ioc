use std::io;

use tokio_core::reactor::Core;

use super::ioc_test;
use super::ioc_test::IocTest;

error_chain! {
    links {
        TestError(ioc_test::Error, ioc_test::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
    }
}

fn test_enable_channel_output(test: &mut IocTest) {
    test.when("OUTPut1 ON").reply_with("");

    test.set_variable("channelOutput-Sel", "ON");
}

pub fn run_test() -> Result<()> {
    let mut reactor = Core::new()?;
    let port = 55000;

    let mut test = IocTest::new(reactor.handle(), port)?;

    test_enable_channel_output(&mut test);

    reactor.run(test)?;

    Ok(())
}
