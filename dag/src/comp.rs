use std::io::Cursor;

use byteorder::*; // FIXME

use sig;

use Address;
use DagComponent;
use DagNode;
use DecodeError;

#[derive(Clone)]
pub struct Signed<T> where T: DagComponent {
    signature: sig::Signature,
    body: T
}

impl<T> Signed<T> where T: DagComponent {

    pub fn new(kp: sig::Keypair, body: T) -> Signed<T> {
        Signed {
            signature: kp.sign(body.get_hash()),
            body: body
        }
    }

    pub fn extract(self) -> T {
        self.body
    }

}

impl<T> DagComponent for Signed<T> where T: DagComponent {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {
        match sig::Signature::from_blob(blob) {
            Ok((sig, len)) => match T::from_blob(&blob[len..]) {
                Ok((b, bl)) => Ok((Signed { signature: sig, body: b }, len + bl)),
                _ => Err(DecodeError)
            },
            _ => Err(DecodeError)
        }
    }

    fn to_blob(&self) -> Vec<u8> {

        let mut buf = Vec::new();
        buf.append(&mut self.signature.to_blob());
        buf.append(&mut self.body.to_blob());
        buf

    }

}

#[derive(Clone)]
pub struct Block {
    version: u32,
    timestamp: i64,
    parents: Vec<Address>,
    segments: Vec<Signed<Segment>>
}

impl DagComponent for Block {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);

        let vr = c.read_u32::<BigEndian>();
        let tsr = c.read_i64::<BigEndian>();
        let pcr = c.read_u8();
        let scr = c.read_u16::<BigEndian>();

        match (vr, tsr, pcr, scr) {
            (Ok(ver), Ok(ts), Ok(parent_cnt), Ok(seg_cnt)) => {

                let mut parents = Vec::new();
                let mut segments = Vec::new();
                let cpos = c.position() as usize;
                let mut inner = &c.into_inner()[cpos..];

                let mut consumed = 0;

                for _ in 0..parent_cnt {
                    match Address::from_blob(inner) {
                        Ok((addr, len)) => {
                            parents.push(addr);
                            inner = &inner[len..];
                            consumed += len;
                        },
                        _ => return Err(DecodeError)
                    }
                }

                for _ in 0..seg_cnt {
                    match Signed::from_blob(inner) {
                        Ok((seg, len)) => {
                            segments.push(seg);
                            inner = &inner[len..];
                            consumed += len;
                        },
                        _ => return Err(DecodeError)
                    }
                }

                Ok((Block {
                    version: ver,
                    timestamp: ts,
                    parents: parents,
                    segments: segments
                }, consumed + cpos))

            },
            (_, _, _, _) => Err(DecodeError)
        }

    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_u32::<BigEndian>(self.version).unwrap();
        v.write_i64::<BigEndian>(self.timestamp).unwrap();
        v.write_u8(self.parents.len() as u8).unwrap();
        v.write_u16::<BigEndian>(self.segments.len() as u16).unwrap();

        for p in &self.parents {
            v.append(&mut p.to_blob());
        }

        for seg in &self.segments {
            v.append(&mut seg.to_blob());
        }

        v

    }

}

impl DagNode for Signed<Block> {

    fn version(&self) -> u32 {
        self.body.version
    }

    fn timestamp(&self) -> i64 {
        self.body.timestamp
    }

}

#[derive(Clone)]
pub enum SegmentContent {
    IdentDecl(sig::Hash),
    Artifact(ArtifactData),
    ArtifactPointer(Address)
}

impl SegmentContent {
    fn specifier(&self) -> u8 {
        use self::SegmentContent::*;
        match self {
            &IdentDecl(_) => 0,
            &Artifact(_) => 1,
            &ArtifactPointer(_) => 2
        }
    }
}

impl DagComponent for SegmentContent {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        use self::SegmentContent::*;

        let mut c = Cursor::new(blob);
        let read = c.position() as usize;
        let tag = c.read_u8().map_err(|_| DecodeError)?;
        let dblob = &c.into_inner()[read..];

        match tag {
            0 => {
                if dblob.len() < sig::SHA256_WIDTH {
                    return Err(DecodeError);
                }

                let mut hd = [0; sig::SHA256_WIDTH];
                for i in 0..sig::SHA256_WIDTH {
                    hd[i] = dblob[i];
                }
                Ok((IdentDecl(sig::Hash::new(hd)), read + sig::SHA256_WIDTH))
            }

            1 => {
                let (ad,len) = ArtifactData::from_blob(dblob)?;
                Ok((Artifact(ad), read + len))
            }

            2 => {
                let (addr,len) = Address::from_blob(dblob)?;
                Ok((ArtifactPointer(addr), read + len))
            }

            _ => Err(DecodeError),
        }
    }

    fn to_blob(&self) -> Vec<u8> {

        use self::SegmentContent::*;

        let mut v = Vec::new();
        v.push(self.specifier());
        match self {
            &IdentDecl(hash) => v.extend_from_slice(&hash.into_array()),
            &Artifact(ref a) => v.append(&mut a.to_blob()),
            &ArtifactPointer(p) => v.append(&mut p.to_blob())
        }

        v

    }

}

#[derive(Clone)]
pub struct Segment {
    timestamp: i64,
    content: SegmentContent
}

impl DagComponent for Segment {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);
        let tsr = c.read_i64::<BigEndian>();
        match tsr {
            Ok(ts) => {

                let read = c.position() as usize;
                match SegmentContent::from_blob(&c.into_inner()[read..]) {
                    Ok((d, sclen)) => Ok((Segment {
                        timestamp: ts,
                        content: d
                    }, read + sclen)),
                    _ => Err(DecodeError)
                }

            },
            _ => Err(DecodeError)
        }

    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_i64::<BigEndian>(self.timestamp).unwrap();
        v.append(&mut self.content.to_blob());
        v

    }

}

#[derive(Clone)]
pub struct ArtifactData {
    spec: u16, // Artifact code.  Big-endian 0xXXXY, where X is the namespace and Y is the subtype.
    body: Vec<u8> // Actual artifact format is specified in a higher layer.
}

impl DagComponent for ArtifactData {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);
        let sr = c.read_u16::<BigEndian>();
        let lr = c.read_u32::<BigEndian>();
        match (sr, lr) {
            (Ok(spec), Ok(len)) => {

                let read = c.position() as usize;
                let buf = &c.into_inner()[read .. read + len as usize];
                let mut v = Vec::new();
                v.extend_from_slice(buf);
                Ok((ArtifactData { spec: spec, body: v }, read + len as usize))

            },
            _ => Err(DecodeError)
        }

    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_u16::<BigEndian>(self.spec).unwrap();
        v.write_u32::<BigEndian>(self.body.len() as u32).unwrap();
        v.extend_from_slice(self.body.as_slice());
        v

    }

}

#[derive(Clone)]
pub struct ArtifactContainer {
    version: u32,
    timestamp: i64,
    content: SegmentContent
}

impl DagComponent for ArtifactContainer {

    fn from_blob(blob: &[u8]) -> Result<(Self, usize), DecodeError> {

        let mut c = Cursor::new(blob);
        let vr = c.read_u32::<BigEndian>();
        let tsr = c.read_i64::<BigEndian>();
        match (vr, tsr) {
            (Ok(ver), Ok(ts)) => {

                let read = c.position() as usize;
                match SegmentContent::from_blob(&c.into_inner()[read..]) {
                    Ok((sc, slen)) => Ok((ArtifactContainer {
                        version: ver,
                        timestamp: ts,
                        content: sc
                    }, read + slen)),
                    _ => Err(DecodeError)
                }

            },
            _ => Err(DecodeError)
        }

    }

    fn to_blob(&self) -> Vec<u8> {

        let mut v = Vec::new();
        v.write_u32::<BigEndian>(self.version).unwrap();
        v.write_i64::<BigEndian>(self.timestamp).unwrap();
        v.append(&mut self.content.to_blob());
        v

    }

}

impl DagNode for Signed<ArtifactContainer> {

    fn version(&self) -> u32 {
        self.body.version
    }

    fn timestamp(&self) -> i64 {
        self.body.timestamp
    }

}
