mod ack_nack;
mod data;

pub struct SubMessage {
    header: Header,
    elements: Vec<Element>,
}

mod submessage_kinds {
    #[non_exhaustive]
    pub enum Entity {
        /// Contains information regarding the value of an application
        /// data-object. Data Submessages are sent by Writers to
        /// Readers.
        Data,

        /// Equivalent to Data, but only contains a part of the new value (one
        /// or more fragments). Allows data to be transmitted as
        /// multiple fragments to overcome transport message size
        /// limitations.
        DataFrag,

        /// Describes the information that is available in a Writer. Heartbeat
        /// messages are sent by a Writer to one or more Readers.
        Heartbeat,

        /// HeartbeatFrag: For fragmented data, describes what fragments are
        /// available in a Writer. HeartbeatFrag messages are sent by a Writer
        /// to one or more Readers.
        HeartbeatFrag,

        /// Describes the information that is no longer relevant to Readers. Gap
        /// messages are sent by a Writer to one or more Readers.
        Gap,

        /// Provides information on the state of a Reader to a Writer. AckNack
        /// messages are sent by a Reader to one or more Writers.
        Acknack,

        /// Provides information on the state of a Reader to a Writer, more
        /// specifically what fragments the Reader is still missing. NackFrag
        /// messages are sent by a Reader to one or more Writers.
        NackFrag,
    }

    pub enum Intepreter {
        /// Provides additional information that logically belongs in the RTPS
        /// Header. The additional information is included inside this
        /// submessage, instead of the HTPS Header, in order to preserve
        /// interoperability with earlier versions of the RTPS protocol. RTPS
        /// version 2.4 and earlier version are not able to process the
        /// HeaderExtension and will skip this submessage.
        HeaderExtension,

        /// Provides information about the source from which subsequent Entity
        /// Submessages originated. This Submessage is primarily used for
        /// relaying RTPS Submessages. This is not discussed in the
        /// currentspecification.
        InfoSource,

        /// Provides information about the final destination ofsubsequent Entity
        /// Submessages. This Submessage is primarily used for relaying RTPS
        /// Submessages. This is not discussed in the current specification.
        InfoDestination,

        /// Provides information about where to reply to the entities that
        /// appear in subsequent Submessages.
        InfoReply,

        /// Provides a source timestamp for subsequent Entity Submessages.
        InfoTimestamp,

        /// Used to add padding to a Message if needed for memory alignment.
        Pad,
    }
}

#[non_exhaustive]

pub struct Header;
pub struct Element;
