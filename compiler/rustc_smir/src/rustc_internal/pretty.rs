use std::io;

use super::run;
use rustc_middle::ty::TyCtxt;

pub fn write_smir_pretty<'tcx, W: io::Write>(tcx: TyCtxt<'tcx>, w: &mut W) -> io::Result<()> {
    writeln!(
        w,
        "// WARNING: This is highly experimental output it's intended for stable-mir developers only."
    )?;
    writeln!(
        w,
        "// If you find a bug or want to improve the output open a issue at https://github.com/rust-lang/project-stable-mir."
    )?;
    run(tcx, || {
        let items = stable_mir::all_local_items();
        let _ = items.iter().map(|item| -> io::Result<()> { item.dump(w) }).collect::<Vec<_>>();
    });
    Ok(())
}
