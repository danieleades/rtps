# Real-Time Publish Subscribe (RTPS) Protocol

![example workflow](https://github.com/danieleades/rtps/actions/workflows/CI.yml/badge.svg)
[![codecov](https://codecov.io/gh/danieleades/rtps/branch/main/graph/badge.svg?token=zMXM0QthTc)](https://codecov.io/gh/danieleades/rtps)

an implementation of the DDS common wire protocol (RTPS)

The specification for this protocol can be found [here](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF)

## Structure

This crate uses a layered structure to separate the 'platform-independent model' (PIM) of the RTPS specification, the 'platform-specific implementations' (PSM), and the concrete implementations.

This provides maximum extensibility for downstream crates to use the core types of this crate, without committing to a specific implementation, or QOS guarantees.

- the PIM is provided by the [rtps-pim](platform-independent-model) crate
- the UDP-based PSM required by the RTPS specification is provided by the [rtps-psm-udp](platform-specific-models/udp) crate
  - additional PSMs can be added by extending the PIM
- A UDP implementation is provided in the [rtps-udp](implementations/udp)

![workspace diagram](http://www.plantuml.com/plantuml/proxy?src=https://raw.githubusercontent.com/danieleades/rtps/main/resources/workspace.puml?cached=false)

## Contributing

Contributions are very welcome! This crate is brand new, and still in an exploratory phase of development, so expect a lot of churn.