pub mod box_;
pub mod deref_;
pub mod drop_;
pub mod rc_;
pub mod refcell_;
pub mod weak_;

fn main() {
    //box智能指针
    box_::simple_box_deo();
    box_::recursion_demo();
    //解引用智能指针
    deref_::simple_deref_demo();
    deref_::diy_deref_demo();
    deref_::hidden_deref_demo();
    //Drop trait智能指针
    drop_::simple_drop_demo();
    drop_::manual_drop_demo();
    //引用计数智能指针
    rc_::simple_rc_demo();
    //弱引用解决循环依赖
    weak_::reference_cycle_demo();
    weak_::weak_reference_count()
}
