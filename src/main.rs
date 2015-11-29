use std::ffi::CString;
extern crate libc;

pub enum DVDReader {}
pub enum DVDFile {}
pub enum VMGIManagementTable {}
pub enum PGCI {}
pub enum ParentalManagementInfoTable {}
pub enum VTSAttributeTable {}
pub enum TextDataManagerInfo {}
pub enum PGCIUnitTable {}
pub enum CellAddressTable {}
pub enum VOBUAddressMap {}
pub enum VTSIManagementTable {}
pub enum VTSPartOfTitleSearchPtrTable {}
pub enum PGCITable {}
pub enum VTSTimeMapTable {}

/*
The following structure defines an IFO file.  The structure is divided into
two parts, the VMGI, or Video Manager Information, which is read from the
VIDEO_TS.[IFO,BUP] file, and the VTSI, or Video Title Set Information, which
is read in from the VTS_XX_0.[IFO,BUP] files.
*/
pub struct IFOHandle {
    file: *const DVDFile,

    /*const  VMGI */
    vmgi_mat: *const VMGIManagementTable,
    tt_srpt : *const TitleSearchPtrTable,
    first_play_pgc: *const PGCI,
    ptl_mait: *const ParentalManagementInfoTable,
    vts_atrt: *const VTSAttributeTable,
    txtdt_mgi: *const TextDataManagerInfo,

    /*const  Common */
    pgci_ut: *const PGCIUnitTable,
    menu_c_adt: *const CellAddressTable,
    menu_vobu_admap: *const VOBUAddressMap,

    /*const  VTSI */
    vtsi_mat: *const VTSIManagementTable,
    vts_ptt_srpt: *const VTSPartOfTitleSearchPtrTable,
    vts_pgcit: *const PGCITable,
    vts_tmapt: *const VTSTimeMapTable,
    vts_c_adt: *const CellAddressTable,
    vts_vobu_admap: *const VOBUAddressMap,
}
/*
  * PartOfTitle Search Pointer Table.
 typedef struct {
   uint16_t nr_of_srpts;
   uint16_t zero_1;
   uint32_t last_byte;
   title_info_t *title;
 } ATTRIBUTE_PACKED tt_srpt_t;
*/
pub struct TitleSearchPtrTable {
    nr_of_srpts: libc::c_ushort,
    zero_1: libc::c_ushort,
    last_byte: libc::c_uint,
    title: *const TitleInfo,
}

/*
 * Title Information.
typedef struct {
  playback_type_t pb_ty;
  uint8_t  nr_of_angles;
  uint16_t nr_of_ptts;
  uint16_t parental_id;
  uint8_t  title_set_nr;
  uint8_t  vts_ttn;
  uint32_t title_set_sector;
} ATTRIBUTE_PACKED title_info_t;
*/
pub enum TitleInfo {}

#[link(name = "dvdread")]
extern {
    fn DVDOpen(path: *const libc::c_char) -> *const DVDReader;
    fn ifoOpen(dvd: *const DVDReader, title: libc::c_int ) -> *const IFOHandle;
}

fn main() {
    let path = CString::new("/Users/matt/dvd").unwrap();
    unsafe {
        let dvd = DVDOpen(path.as_ptr());
        let ifo = ifoOpen(dvd, 0);
        let title_info = &(*ifo.tt_srpt);
        println!("DVD has {} titles", title_info.nr_of_srpts);
    }
    println!("Not dead yet!");
}
