#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorOffice {
    OfficeClose(u32),
    OfficeNotFound(u32),
    OfficeFull(u32),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct OfficeOne {
    pub next_office: Result<OfficeTwo, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct OfficeTwo {
    pub next_office: Result<OfficeThree, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct OfficeThree {
    pub next_office: Result<OfficeFour, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct OfficeFour {
    pub document_id: Result<u32, ErrorOffice>,
}

impl OfficeOne {
    pub fn get_document_id(&self) -> Result<u32, ErrorOffice> {
        // println!("{:?}", self.next_office.unwrap().next_office.unwrap().next_office.unwrap().document_id);
        let s = self.next_office?.next_office?.next_office?.document_id;
        println!("{:?}", s);
        s
    }
}