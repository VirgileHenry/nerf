use crate::Widget;




pub trait WidgetList {
    const SIZE: usize;
}

pub struct Nil;

impl WidgetList for Nil {
    const SIZE: usize = 0;
}

pub struct Cons<UserEvent, W: Widget<UserEvent>, Rest: WidgetList>(core::marker::PhantomData<(W, Rest, UserEvent)>);

impl<UserEvent, W: Widget<UserEvent>, Rest: WidgetList> WidgetList for Cons<UserEvent, W, Rest> {
    const SIZE: usize = Rest::SIZE + 1;
} 

#[macro_export]
macro_rules! widget_list {
    () => {
        Nil  
    };
    ($widget_type:ty) => {
        Cons::<$widget_type, Nil>
    };
    ($widget_type:ty, $($rest:tt)*) => {
        Cons::<$widget_type, widget_list!($($rest)*)>
    };
}
