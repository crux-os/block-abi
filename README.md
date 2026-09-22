# block-abi

Block-device IPC ABI shared across virtio-blk / ahci / nvme / ide
drivers -- extracted from km-virtio-blk into its own repository so
consumers depend on the stable contract, never on driver
implementation details, and the two can version independently.
