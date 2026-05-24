pub(crate) const RTS_USER_SIGNALS: u32 = 1;

pub(crate) const MAX_N_CAPABILITIES: usize = 256;

pub(crate) const CACHELINE_SIZE: usize = cfg_select! {
    target_arch = "s390x" => 256,
    target_arch = "aarch64" => 128,
    target_arch = "x86_64" => 64,
    _ => 32,
};
