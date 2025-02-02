// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use crate::common;
use crate::common::spdm_codec::SpdmCodec;
use crate::config;
use codec::{enum_builder, Codec, Reader, Writer};

enum_builder! {
    @U16
    EnumName: RegistryOrStandardsBodyID;
    EnumVal{
        DMTF => 0x00,
        TCG => 0x01,
        USB => 0x02,
        PCISIG => 0x03,
        IANA => 0x04,
        HDBASET => 0x05,
        MIPI => 0x06,
        CXL => 0x07,
        JEDEC => 0x08
    }
}

impl RegistryOrStandardsBodyID {
    pub fn get_default_vendor_id_len(&self) -> u16 {
        match self {
            RegistryOrStandardsBodyID::DMTF => 0,
            RegistryOrStandardsBodyID::TCG => 2,
            RegistryOrStandardsBodyID::USB => 2,
            RegistryOrStandardsBodyID::PCISIG => 2,
            RegistryOrStandardsBodyID::IANA => 4,
            RegistryOrStandardsBodyID::HDBASET => 4,
            RegistryOrStandardsBodyID::MIPI => 2,
            RegistryOrStandardsBodyID::CXL => 2,
            RegistryOrStandardsBodyID::JEDEC => 2,
            RegistryOrStandardsBodyID::Unknown(_) => 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VendorIDStruct {
    pub len: u8,
    pub vendor_id: [u8; config::MAX_SPDM_VENDOR_DEFINED_VENDOR_ID_LEN],
}

impl Codec for VendorIDStruct {
    fn encode(&self, bytes: &mut Writer) {
        self.len.encode(bytes);
        for d in self.vendor_id.iter().take(self.len as usize) {
            d.encode(bytes);
        }
    }

    fn read(r: &mut Reader) -> Option<VendorIDStruct> {
        let len = u8::read(r)?;
        let mut vendor_id = [0u8; config::MAX_SPDM_VENDOR_DEFINED_VENDOR_ID_LEN];
        for d in vendor_id.iter_mut().take(len as usize) {
            *d = u8::read(r)?;
        }
        Some(VendorIDStruct { len, vendor_id })
    }

    fn read_bytes(bytes: &[u8]) -> Option<Self> {
        let mut rd = Reader::init(bytes);
        Self::read(&mut rd)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ReqPayloadStruct {
    pub req_length: u16,
    pub vendor_defined_req_payload: [u8; config::MAX_SPDM_VENDOR_DEFINED_PAYLOAD_SIZE],
}
impl Codec for ReqPayloadStruct {
    fn encode(&self, bytes: &mut Writer) {
        self.req_length.encode(bytes);
        for d in self
            .vendor_defined_req_payload
            .iter()
            .take(self.req_length as usize)
        {
            d.encode(bytes);
        }
    }

    fn read(r: &mut Reader) -> Option<ReqPayloadStruct> {
        let req_length = u16::read(r)?;
        let mut vendor_defined_req_payload = [0u8; config::MAX_SPDM_VENDOR_DEFINED_PAYLOAD_SIZE];
        for d in vendor_defined_req_payload
            .iter_mut()
            .take(req_length as usize)
        {
            *d = u8::read(r)?;
        }
        Some(ReqPayloadStruct {
            req_length,
            vendor_defined_req_payload,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ResPayloadStruct {
    pub res_length: u16,
    pub vendor_defined_res_payload: [u8; config::MAX_SPDM_VENDOR_DEFINED_PAYLOAD_SIZE],
}

impl Codec for ResPayloadStruct {
    fn encode(&self, bytes: &mut Writer) {
        self.res_length.encode(bytes);
        for d in self
            .vendor_defined_res_payload
            .iter()
            .take(self.res_length as usize)
        {
            d.encode(bytes);
        }
    }

    fn read(r: &mut Reader) -> Option<ResPayloadStruct> {
        let res_length = u16::read(r)?;
        let mut vendor_defined_res_payload = [0u8; config::MAX_SPDM_VENDOR_DEFINED_PAYLOAD_SIZE];
        for d in vendor_defined_res_payload
            .iter_mut()
            .take(res_length as usize)
        {
            *d = u8::read(r)?;
        }
        Some(ResPayloadStruct {
            res_length,
            vendor_defined_res_payload,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SpdmVendorDefinedRequestPayload {
    pub standard_id: RegistryOrStandardsBodyID,
    pub vendor_id: VendorIDStruct,
    pub req_payload: ReqPayloadStruct,
}

impl SpdmCodec for SpdmVendorDefinedRequestPayload {
    fn spdm_encode(&self, _context: &mut common::SpdmContext, bytes: &mut Writer) {
        0u8.encode(bytes); // param1
        0u8.encode(bytes); // param2
        self.standard_id.encode(bytes); //Standard ID
        self.vendor_id.encode(bytes);
        self.req_payload.encode(bytes);
    }

    fn spdm_read(
        _context: &mut common::SpdmContext,
        r: &mut Reader,
    ) -> Option<SpdmVendorDefinedRequestPayload> {
        u8::read(r)?; // param1
        u8::read(r)?; // param2
        let standard_id = RegistryOrStandardsBodyID::read(r)?; // Standard ID
        let vendor_id = VendorIDStruct::read(r)?;
        let req_payload = ReqPayloadStruct::read(r)?;

        Some(SpdmVendorDefinedRequestPayload {
            standard_id,
            vendor_id,
            req_payload,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SpdmVendorDefinedResponsePayload {
    pub standard_id: RegistryOrStandardsBodyID,
    pub vendor_id: VendorIDStruct,
    pub res_payload: ResPayloadStruct,
}

impl SpdmCodec for SpdmVendorDefinedResponsePayload {
    fn spdm_encode(&self, _context: &mut common::SpdmContext, bytes: &mut Writer) {
        0u8.encode(bytes); // param1
        0u8.encode(bytes); // param2
        self.standard_id.encode(bytes); //Standard ID
        self.vendor_id.encode(bytes);
        self.res_payload.encode(bytes);
    }

    fn spdm_read(
        _context: &mut common::SpdmContext,
        r: &mut Reader,
    ) -> Option<SpdmVendorDefinedResponsePayload> {
        u8::read(r)?; // param1
        u8::read(r)?; // param2
        let standard_id = RegistryOrStandardsBodyID::read(r)?; // Standard ID
        let vendor_id = VendorIDStruct::read(r)?;
        let res_payload = ResPayloadStruct::read(r)?;

        Some(SpdmVendorDefinedResponsePayload {
            standard_id,
            vendor_id,
            res_payload,
        })
    }
}
