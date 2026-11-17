//! Integration tests for SigmaOS-arch-system-configs-backup.

use sigmaos_arch_system_configs_backup_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
