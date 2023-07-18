use std::num::NonZeroU32;



/// Size requirements a widget can have.
/// Here, widgets have no minimum sizes, as the display size can be as small as possible.
/// Widgets can require to have a fix size, or a maximum size.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WidgetSizeRequirement {
    /// The widget request a fixed size.
    Fixed(NonZeroU32),
    /// The widget request a minimum size, but can be expanded.
    /// This requirement is a soft requirement, and can be ignored.
    /// However, in debug mode, size overflow will be checked.
    Min(NonZeroU32),
    /// The widget request a maximum size, but can be reduced.
    Max(NonZeroU32),
    /// The widget request a minimum and a maximum size.
    /// The min requirement is a soft requirement, and can be ignored.
    /// However, in debug mode, size overflow will be checked.
    MinMax(NonZeroU32, NonZeroU32),
    /// The widget have no size constraints, and will fill up all the available space.
    /// The given value is the flex value, and tells how to distribute the remaining space.
    Flex(NonZeroU32),
    /// The widget have no size constraints.
    /// In most use cases, this will tell that there is no widget.
    None,
}

impl std::ops::BitOr for WidgetSizeRequirement {
    type Output = Self;
    /// The bitor between two size requirements is as if they were next to eachother, and we are looking for a requirement that fits both.
    /// 
    /// +---- Bit Or requirement ----> 
    /// |  +--------+
    /// |  |        |
    /// |  |        |
    /// |  +--------+
    /// |  +------------+
    /// |  |            |
    /// |  |            |
    /// |  +------------+
    /// v
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (WidgetSizeRequirement::None, other) |
            (other, WidgetSizeRequirement::None) => other,
            (WidgetSizeRequirement::Flex(_), WidgetSizeRequirement::Flex(_)) |
            (WidgetSizeRequirement::Flex(_), WidgetSizeRequirement::Max(_)) |
            (WidgetSizeRequirement::Max(_), WidgetSizeRequirement::Flex(_)) => WidgetSizeRequirement::Flex(unsafe { NonZeroU32::new_unchecked(1) }),
            (WidgetSizeRequirement::Flex(_), WidgetSizeRequirement::Fixed(size)) |
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::Flex(_)) |
            (WidgetSizeRequirement::Flex(_), WidgetSizeRequirement::Min(size)) |
            (WidgetSizeRequirement::Min(size), WidgetSizeRequirement::Flex(_)) => WidgetSizeRequirement::Min(size),
            (WidgetSizeRequirement::Flex(_), WidgetSizeRequirement::MinMax(min, _)) |
            (WidgetSizeRequirement::MinMax(min, _), WidgetSizeRequirement::Flex(_)) => WidgetSizeRequirement::Min(min),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::Fixed(other_size)) => WidgetSizeRequirement::Fixed(size.max(other_size)),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::Min(other_size)) |
            (WidgetSizeRequirement::Min(other_size), WidgetSizeRequirement::Fixed(size)) => WidgetSizeRequirement::Min(size.max(other_size)),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::Max(other_size)) |
            (WidgetSizeRequirement::Max(other_size), WidgetSizeRequirement::Fixed(size)) => WidgetSizeRequirement::Max(size.max(other_size)),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::MinMax(other_min, other_max)) |
            (WidgetSizeRequirement::MinMax(other_min, other_max), WidgetSizeRequirement::Fixed(size)) => WidgetSizeRequirement::MinMax(size.min(other_min), size.max(other_max)),
            (WidgetSizeRequirement::Min(size), WidgetSizeRequirement::Min(other_size)) => WidgetSizeRequirement::Min(size.min(other_size)),
            (WidgetSizeRequirement::Min(size), WidgetSizeRequirement::Max(max)) |
            (WidgetSizeRequirement::Max(max), WidgetSizeRequirement::Min(size)) => WidgetSizeRequirement::MinMax(size, max),
            (WidgetSizeRequirement::Min(size), WidgetSizeRequirement::MinMax(other_min, max)) |
            (WidgetSizeRequirement::MinMax(other_min, max), WidgetSizeRequirement::Min(size)) => WidgetSizeRequirement::MinMax(size.min(other_min), max),
            (WidgetSizeRequirement::Max(size), WidgetSizeRequirement::Max(other_size)) => WidgetSizeRequirement::Max(size.max(other_size)),
            (WidgetSizeRequirement::Max(size), WidgetSizeRequirement::MinMax(other_min, other_max)) |
            (WidgetSizeRequirement::MinMax(other_min, other_max), WidgetSizeRequirement::Max(size)) => WidgetSizeRequirement::MinMax(other_min, size.max(other_max)),
            (WidgetSizeRequirement::MinMax(min, max), WidgetSizeRequirement::MinMax(other_min, other_max)) => WidgetSizeRequirement::MinMax(min.min(other_min), max.max(other_max)),
        }
    }
}

impl std::ops::BitAnd for WidgetSizeRequirement {
    type Output = Self;
    /// The bitand between two size requirements is as if they were following each other, and we are looking for a requirement that fits the sum of them.
    /// 
    /// +---- Bit And requirement ----> 
    /// |  +--------+ +------------+
    /// |  |        | |            |
    /// |  |        | |            |
    /// |  +--------+ +------------+
    /// |
    /// v
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (WidgetSizeRequirement::None, other) |
            (other, WidgetSizeRequirement::None) => other,
            (WidgetSizeRequirement::Flex(_), _) |
            (_, WidgetSizeRequirement::Flex(_)) => WidgetSizeRequirement::Flex(unsafe { NonZeroU32::new_unchecked(1) }),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::Fixed(other_size)) => WidgetSizeRequirement::Fixed(size.saturating_add(other_size.get())),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::Min(other_size)) |
            (WidgetSizeRequirement::Min(other_size), WidgetSizeRequirement::Fixed(size)) => WidgetSizeRequirement::Min(size.saturating_add(other_size.get())),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::Max(other_size)) |
            (WidgetSizeRequirement::Max(other_size), WidgetSizeRequirement::Fixed(size)) => WidgetSizeRequirement::MinMax(size, size.saturating_add(other_size.get())),
            (WidgetSizeRequirement::Fixed(size), WidgetSizeRequirement::MinMax(other_min, other_max)) |
            (WidgetSizeRequirement::MinMax(other_min, other_max), WidgetSizeRequirement::Fixed(size)) => WidgetSizeRequirement::MinMax(size.saturating_add(other_min.get()), size.saturating_add(other_max.get())),
            (WidgetSizeRequirement::Min(size), WidgetSizeRequirement::Min(other_size)) => WidgetSizeRequirement::Min(size.saturating_add(other_size.get())),
            (WidgetSizeRequirement::Min(size), WidgetSizeRequirement::Max(_)) |
            (WidgetSizeRequirement::Max(_), WidgetSizeRequirement::Min(size)) => WidgetSizeRequirement::Min(size),
            (WidgetSizeRequirement::Min(size), WidgetSizeRequirement::MinMax(other_min, _)) |
            (WidgetSizeRequirement::MinMax(other_min, _), WidgetSizeRequirement::Min(size)) => WidgetSizeRequirement::Min(size.saturating_add(other_min.get())),
            (WidgetSizeRequirement::Max(size), WidgetSizeRequirement::Max(other_size)) => WidgetSizeRequirement::Max(size.saturating_add(other_size.get())),
            (WidgetSizeRequirement::Max(size), WidgetSizeRequirement::MinMax(_, other_max)) |
            (WidgetSizeRequirement::MinMax(_, other_max), WidgetSizeRequirement::Max(size)) => WidgetSizeRequirement::Max(size.saturating_add(other_max.get())),
            (WidgetSizeRequirement::MinMax(min, max), WidgetSizeRequirement::MinMax(other_min, other_max)) => WidgetSizeRequirement::MinMax(min.saturating_add(other_min.get()), max.saturating_add(other_max.get())),
        }
    }
}


impl WidgetSizeRequirement {
    pub fn distribute_available_size<const N: usize>(requirements: [WidgetSizeRequirement; N], available_space: NonZeroU32) -> [u32; N] {

        let min_size = get_min_size(&requirements);

        if min_size > available_space.get() {
            if cfg!(debug_assertions) {
                println!("Overflow : Not enough space ({}) to meet the minimum size requirements ({}) of combined requirements.", available_space, min_size);
            }
            fill_available_space(requirements, available_space, min_size)
        }
        else {
            let mut sizes = [0; N];
            let mut remaining_space = available_space.get();
            // assign min values first !
            for (i, requirement) in requirements.iter().enumerate() {
                match requirement {
                    WidgetSizeRequirement::Fixed(size) => {
                        sizes[i] = size.get();
                        remaining_space -= sizes[i];
                    },
                    _ => {},
                }
            }
            // check flex values
            let flex_space = get_flex_space(&requirements, remaining_space);
            // assign every min values that are bigger than the flex space
            for (i, requirement) in requirements.iter().enumerate() {
                match requirement {
                    WidgetSizeRequirement::MinMax(size, _) |
                    WidgetSizeRequirement::Min(size) => if size.get() > flex_space {
                        sizes[i] = size.get();
                        remaining_space -= sizes[i];
                    },
                    _ => {},
                }
            }
            loop {
                // while there are max that are too small for the flex space, assign them and recompute
                let flex_space = get_flex_space(&requirements, remaining_space);
                let mut all_max_assigned = true;
                for (i, requirement) in requirements.iter().enumerate() {
                    match requirement {
                        WidgetSizeRequirement::Max(max) |
                        WidgetSizeRequirement::MinMax(_, max) => if sizes[i] == 0 && max.get() < flex_space {
                            // widget cannot grow to fill flex space
                            sizes[i] = max.get();
                            all_max_assigned = false;
                        },
                        _ => {},
                    }
                }
                if all_max_assigned {
                    break;
                }
            }
            // assign flex values
            let flex_space = get_flex_space(&requirements, remaining_space);
            for (i, requirement) in requirements.iter().enumerate() {
                match requirement {
                    WidgetSizeRequirement::Max(_) |
                    WidgetSizeRequirement::MinMax(_, _) |
                    WidgetSizeRequirement::Min(_) |
                    WidgetSizeRequirement::Flex(_) => if sizes[i] == 0 {
                        sizes[i] = flex_space;
                        remaining_space -= flex_space;
                    },
                    _ => {},
                }
            }
            // finally, it may remains pixels because of euclidian div: assign it to the widgets
            while remaining_space > 0 {
                for (i, requirement) in requirements.iter().enumerate() {
                    match requirement {
                        WidgetSizeRequirement::Max(max) |
                        WidgetSizeRequirement::MinMax(_, max)  => if sizes[i] + remaining_space <= max.get() {
                            sizes[i] += 1.min(remaining_space);
                            remaining_space -= 1.min(remaining_space);
                        }
                        WidgetSizeRequirement::Min(_) |
                        WidgetSizeRequirement::Flex(_) => {
                            sizes[i] += 1.min(remaining_space);
                            remaining_space -= 1.min(remaining_space);
                        },
                        _ => {},
                    }
                }
            }

            sizes
        }
    }
}


fn get_min_size(requirements: &[WidgetSizeRequirement]) -> u32 {
    requirements.iter().map(|requirement| {
        match requirement {
            WidgetSizeRequirement::Fixed(size) |
            WidgetSizeRequirement::Min(size) |
            WidgetSizeRequirement::MinMax(size, _) => size.get(),
            WidgetSizeRequirement::Max(_) |
            WidgetSizeRequirement::Flex(_) |
            WidgetSizeRequirement::None => 0,
        }
    }).sum()
}

fn fill_available_space<const N: usize>(requirements: [WidgetSizeRequirement; N], available_space: NonZeroU32, min_size: u32) -> [u32; N] {
    let mut remaining_space = available_space.get();
    let mut sizes = [0; N];

    for (i, requirement) in requirements.iter().enumerate() {
        match requirement {
            WidgetSizeRequirement::Fixed(size) |
            WidgetSizeRequirement::Min(size) |
            WidgetSizeRequirement::MinMax(size, _) => {
                sizes[i] = size.get() * available_space.get() / min_size;
                remaining_space -= sizes[i];
            },
            _ => {},
        }
    }
    // finally, it may remains a pixel because of euclidian div: assign it to the first widget
    while remaining_space > 0 {
        for (i, requirement) in requirements.iter().enumerate() {
            match requirement {
                WidgetSizeRequirement::Fixed(_) |
                WidgetSizeRequirement::Min(_) |
                WidgetSizeRequirement::MinMax(_, _) => {
                    sizes[i] += 1.min(remaining_space);
                    remaining_space -= 1.min(remaining_space);
                },
                _ => {},
            }
        }
    }

    sizes
}

fn get_flex_space(requirements: &[WidgetSizeRequirement], remaining_space: u32) -> u32 {
    let flex_count = requirements.iter().map(|requirement| match requirement {
        WidgetSizeRequirement::Min(_) |
        WidgetSizeRequirement::Max(_) |
        WidgetSizeRequirement::MinMax(_, _) => 1,
        WidgetSizeRequirement::Flex(flex) => flex.get(),
        _ => 0,
    }).sum::<u32>();
    if flex_count == 0 {
        0
    }
    else {
        remaining_space / flex_count
    }
}