use std::num::NonZeroU32;



/// Size requirements a widget can have.
/// Here, widgets have no minimum sizes, as the display size can be as small as possible.
/// Widgets can require to have a fix size, or a maximum size.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WidgetSizeRequirement {
    /// The widget request a fixed size.
    Fixed {
        size: NonZeroU32,
    },
    /// The widget request a minimum size, but can be expanded.
    /// This requirement is a soft requirement, and can be ignored.
    /// However, in debug mode, size overflow will be checked.
    Min {
        min: NonZeroU32,
        flex: NonZeroU32,
    },
    /// The widget request a maximum size, but can be reduced.
    Max {
        max: NonZeroU32,
        flex: NonZeroU32,
    },
    /// The widget request a minimum and a maximum size.
    /// The min requirement is a soft requirement, and can be ignored.
    /// However, in debug mode, size overflow will be checked.
    MinMax {
        min: NonZeroU32,
        max: NonZeroU32,
        flex: NonZeroU32,
    },
    /// The widget have no size constraints, and will fill up all the available space.
    /// The given value is the flex value, and tells how to distribute the remaining space.
    Flex {
        flex: NonZeroU32,
    },
    /// The widget have no size constraints.
    /// In most use cases, this will tell that there is no widget.
    /// This is the same as a fixed size of 0, or a flex of 0.
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
            // None 
            (WidgetSizeRequirement::None, WidgetSizeRequirement::None) => WidgetSizeRequirement::None,
            (other, WidgetSizeRequirement::None) => other,
            (WidgetSizeRequirement::None, other) => other,
            // Fixed, other
            (WidgetSizeRequirement::Fixed { size: s1 }, WidgetSizeRequirement::Fixed { size: s2 }) =>
                WidgetSizeRequirement::Fixed { size: s1.max(s2) },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::Flex { flex }) =>
                WidgetSizeRequirement::Min { min: size, flex },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::Min { min, flex }) =>
                WidgetSizeRequirement::Min { min: size.saturating_add(min.get()), flex },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::Max { max, flex }) =>
                WidgetSizeRequirement::MinMax { min: size, max: max.saturating_add(size.get()), flex, },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::MinMax { min, max, flex }) =>
                WidgetSizeRequirement::MinMax { min: min.saturating_add(size.get()), max: max.saturating_add(size.get()), flex },
            // Flex, other
            (WidgetSizeRequirement::Flex { flex }, WidgetSizeRequirement::Fixed { size }) =>
                WidgetSizeRequirement::Min { min: size, flex },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Flex { flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::Min { min, flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::Max { flex: f2, .. }) =>
                WidgetSizeRequirement::Flex { flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::MinMax { min, flex: f2, .. }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get())},
            // Min, other
            (WidgetSizeRequirement::Min { min, flex }, WidgetSizeRequirement::Fixed { size }) => 
                WidgetSizeRequirement::Min { min: min.saturating_add(size.get()), flex },
            (WidgetSizeRequirement::Min { min, flex: f1 }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Min { min: m1, flex: f1 }, WidgetSizeRequirement::Min { min: m2, flex: f2 }) =>
                WidgetSizeRequirement::Min { min: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Min { min, flex: f1 }, WidgetSizeRequirement::Max { flex: f2, .. }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Min { min: m1, flex: f1 }, WidgetSizeRequirement::MinMax { min: m2, flex: f2, .. }) =>
                WidgetSizeRequirement::Min { min: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            // Max, other
            (WidgetSizeRequirement::Max { max, flex }, WidgetSizeRequirement::Fixed { size }) =>
                WidgetSizeRequirement::Max { max: max.saturating_add(size.get()), flex },
            (WidgetSizeRequirement::Max { flex: f1, .. }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Flex { flex: f1.saturating_add(f2.get())},
            (WidgetSizeRequirement::Max { flex: f1, .. }, WidgetSizeRequirement::Min { min, flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Max { max: m1, flex: f1 }, WidgetSizeRequirement::Max { max: m2, flex: f2 }) =>
                WidgetSizeRequirement::Max { max: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Max { max: m1, flex: f1 }, WidgetSizeRequirement::MinMax { min, max: m2, flex: f2 }) =>
                WidgetSizeRequirement::MinMax { min, max: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            // MinMax, other
            (WidgetSizeRequirement::MinMax { min, max, flex }, WidgetSizeRequirement::Fixed { size }) =>
                WidgetSizeRequirement::MinMax { min: min.saturating_add(size.get()), max: max.saturating_add(size.get()), flex },
            (WidgetSizeRequirement::MinMax { min, flex: f1, .. }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::MinMax { min: m1, flex: f1, .. }, WidgetSizeRequirement::Min { min: m2, flex: f2 }) =>
                WidgetSizeRequirement::Min { min: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::MinMax { min, max: m1, flex: f1 }, WidgetSizeRequirement::Max { max: m2, flex: f2 }) =>
                WidgetSizeRequirement::MinMax { min, max: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::MinMax { min: mi1, max: ma1, flex: f1 }, WidgetSizeRequirement::MinMax { min: mi2, max: ma2, flex: f2 }) =>
            WidgetSizeRequirement::MinMax { min: mi1.saturating_add(mi2.get()), max: ma1.saturating_add(ma2.get()), flex: f1.saturating_add(f2.get()) },
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
            // None 
            (WidgetSizeRequirement::None, WidgetSizeRequirement::None) => WidgetSizeRequirement::None,
            (other, WidgetSizeRequirement::None) => other,
            (WidgetSizeRequirement::None, other) => other,
            // Fixed, other
            (WidgetSizeRequirement::Fixed { size: s1 }, WidgetSizeRequirement::Fixed { size: s2 }) =>
                WidgetSizeRequirement::Fixed { size: s1.saturating_add(s2.get()) },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::Flex { flex }) =>
                WidgetSizeRequirement::Min { min: size, flex },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::Min { min, flex }) =>
                WidgetSizeRequirement::Min { min: size.saturating_add(min.get()), flex },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::Max { max, flex }) =>
                WidgetSizeRequirement::MinMax { min: size, max: max.saturating_add(size.get()), flex, },
            (WidgetSizeRequirement::Fixed { size }, WidgetSizeRequirement::MinMax { min, max, flex }) =>
                WidgetSizeRequirement::MinMax { min: min.saturating_add(size.get()), max: max.saturating_add(size.get()), flex },
            // Flex, other
            (WidgetSizeRequirement::Flex { flex }, WidgetSizeRequirement::Fixed { size }) =>
                WidgetSizeRequirement::Min { min: size, flex },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Flex { flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::Min { min, flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::Max { flex: f2, .. }) =>
                WidgetSizeRequirement::Flex { flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Flex { flex: f1 }, WidgetSizeRequirement::MinMax { min, flex: f2, .. }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get())},
            // Min, other
            (WidgetSizeRequirement::Min { min, flex }, WidgetSizeRequirement::Fixed { size }) => 
                WidgetSizeRequirement::Min { min: min.saturating_add(size.get()), flex },
            (WidgetSizeRequirement::Min { min, flex: f1 }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Min { min: m1, flex: f1 }, WidgetSizeRequirement::Min { min: m2, flex: f2 }) =>
                WidgetSizeRequirement::Min { min: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Min { min, flex: f1 }, WidgetSizeRequirement::Max { flex: f2, .. }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Min { min: m1, flex: f1 }, WidgetSizeRequirement::MinMax { min: m2, flex: f2, .. }) =>
                WidgetSizeRequirement::Min { min: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            // Max, other
            (WidgetSizeRequirement::Max { max, flex }, WidgetSizeRequirement::Fixed { size }) =>
                WidgetSizeRequirement::Max { max: max.saturating_add(size.get()), flex },
            (WidgetSizeRequirement::Max { flex: f1, .. }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Flex { flex: f1.saturating_add(f2.get())},
            (WidgetSizeRequirement::Max { flex: f1, .. }, WidgetSizeRequirement::Min { min, flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Max { max: m1, flex: f1 }, WidgetSizeRequirement::Max { max: m2, flex: f2 }) =>
                WidgetSizeRequirement::Max { max: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::Max { max: m1, flex: f1 }, WidgetSizeRequirement::MinMax { min, max: m2, flex: f2 }) =>
                WidgetSizeRequirement::MinMax { min, max: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            // MinMax, other
            (WidgetSizeRequirement::MinMax { min, max, flex }, WidgetSizeRequirement::Fixed { size }) =>
                WidgetSizeRequirement::MinMax { min: min.saturating_add(size.get()), max: max.saturating_add(size.get()), flex },
            (WidgetSizeRequirement::MinMax { min, flex: f1, .. }, WidgetSizeRequirement::Flex { flex: f2 }) =>
                WidgetSizeRequirement::Min { min, flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::MinMax { min: m1, flex: f1, .. }, WidgetSizeRequirement::Min { min: m2, flex: f2 }) =>
                WidgetSizeRequirement::Min { min: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::MinMax { min, max: m1, flex: f1 }, WidgetSizeRequirement::Max { max: m2, flex: f2 }) =>
                WidgetSizeRequirement::MinMax { min, max: m1.saturating_add(m2.get()), flex: f1.saturating_add(f2.get()) },
            (WidgetSizeRequirement::MinMax { min: mi1, max: ma1, flex: f1 }, WidgetSizeRequirement::MinMax { min: mi2, max: ma2, flex: f2 }) =>
            WidgetSizeRequirement::MinMax { min: mi1.saturating_add(mi2.get()), max: ma1.saturating_add(ma2.get()), flex: f1.saturating_add(f2.get()) },
        }
    }
}

impl core::ops::Add<u32> for WidgetSizeRequirement {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        match self {
            WidgetSizeRequirement::Fixed { size } => WidgetSizeRequirement::Fixed { size: size.saturating_add(rhs) },
            WidgetSizeRequirement::Flex { flex } => match NonZeroU32::new(rhs) {
                Some(min) => WidgetSizeRequirement::Min { min, flex },
                None => WidgetSizeRequirement::Flex { flex },
            },
            WidgetSizeRequirement::Max { max, flex } => match NonZeroU32::new(rhs) {
                Some(min) => WidgetSizeRequirement::MinMax { min, max: max.saturating_add(rhs), flex, },
                None => WidgetSizeRequirement::Max { max, flex },
            },
            WidgetSizeRequirement::Min { min, flex } =>
                WidgetSizeRequirement::Min { min: min.saturating_add(rhs), flex },
            WidgetSizeRequirement::MinMax { min, max, flex } =>
                WidgetSizeRequirement::MinMax { min: min.saturating_add(rhs), max: max.saturating_add(rhs), flex },
            WidgetSizeRequirement::None => match NonZeroU32::new(rhs) {
                Some(min) => WidgetSizeRequirement::Fixed { size: min },
                None => WidgetSizeRequirement::None,
            },
        }
    }
}

impl core::ops::Mul<u32> for WidgetSizeRequirement {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        match (NonZeroU32::new(rhs), self) {
            (Some(rhs), WidgetSizeRequirement::Fixed { size }) => WidgetSizeRequirement::Fixed { size: size.saturating_mul(rhs) },
            (Some(rhs), WidgetSizeRequirement::Flex { flex }) => WidgetSizeRequirement::Flex { flex: flex.saturating_mul(rhs) },
            (Some(rhs), WidgetSizeRequirement::Max { max, flex }) => WidgetSizeRequirement::Max { max: max.saturating_mul(rhs), flex: flex.saturating_mul(rhs) },
            (Some(rhs), WidgetSizeRequirement::Min { min, flex }) => WidgetSizeRequirement::Min { min: min.saturating_mul(rhs), flex: flex.saturating_mul(rhs) },
            (Some(rhs), WidgetSizeRequirement::MinMax { min, max, flex }) => WidgetSizeRequirement::MinMax { min: min.saturating_mul(rhs), max: max.saturating_mul(rhs), flex: flex.saturating_mul(rhs) },
            (None, _) | (Some(_), WidgetSizeRequirement::None) => WidgetSizeRequirement::None,
        }
    }
}

impl WidgetSizeRequirement {
    /// Distribute a given available_space between multiple requirements.
    /// The algorithm will do it's best to respect all provided requirements, but it is sometimes impossible.
    /// todo: rework this whole thing, as it is super primitive for now.
    /// I'm convinced there is a nice mathy elegant way to do it, and I've started searching it.
    pub fn distribute_available_size<const N: usize>(requirements: [WidgetSizeRequirement; N], available_space: NonZeroU32) -> [u32; N] {

        let mut results = [0; N];

        // Step 1: get the number of min space required and flex components

        let (min_requirements, total_flex): (u32, u32) = requirements.iter().map(|req| match req {
            WidgetSizeRequirement::Fixed { size } => (size.get(), 0),
            WidgetSizeRequirement::Flex { flex } => (0, flex.get()),
            WidgetSizeRequirement::Max { flex, .. } => (0, flex.get(), ),
            WidgetSizeRequirement::Min { min, flex } => (min.get(), flex.get()),
            WidgetSizeRequirement::MinMax { min, flex, .. } => (min.get(), flex.get(), ),
            WidgetSizeRequirement::None => (0, 0),
        }).fold((0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2));

        if min_requirements > available_space.get() {
            panic!("TODO: handle case where not enough space for min requirements! ({min_requirements} required, got {available_space})");            
        }

        // Step 2: distribute min requirements

        for (result, requirement) in results.iter_mut().zip(requirements.iter()) {
            match requirement {
                WidgetSizeRequirement::Fixed { size: min } |
                WidgetSizeRequirement::Min { min, .. } |
                WidgetSizeRequirement::MinMax { min, .. } => *result = min.get(),
                _ => {},
            }
        }
        
        // Step 3: distribute flex space

        let mut available_flex_space = available_space.get() - min_requirements;
        let mut total_flex = total_flex;
        let mut progress_have_been_made = true;

        while progress_have_been_made {

            // we keep giving space as long as there is:    
            // - space to give
            // - wigets than can accept more space (flex)
            // - changes that have been made last iteration

            progress_have_been_made = false;
            if available_flex_space == 0 { break; }
            let non_zero_total_flex = match NonZeroU32::new(total_flex) {
                None => break,
                Some(value) => value,
            };
            
            
            // creates an iterator over the space we computed for each flex unit.
            // this iterator will repeat flex_val1 fv1 amount and flex_val2 fv2_amount
            // this way, we are sure to distribute all flex space, taking remainder of euclidian div into account.
            let mut flex_size_it = full_integer_div(available_flex_space, non_zero_total_flex);
            
            for (result, requirement) in results.iter_mut().zip(requirements) {
                match requirement {
                    // for widgets with max property : 
                    // distribute flex, maxing to their max.
                    // if we reach their max, remove their flex as part of the equation.
                    WidgetSizeRequirement::Max { max, flex } | 
                    WidgetSizeRequirement::MinMax { max, flex, .. } => {
                        if *result < max.get() {
                            // get the amount of flex space from the iterator according to our flex value
                            let consumed = flex_size_it.next_flex(flex);
                            if consumed >= *result - max.get() {
                                total_flex -= flex.get();
                                available_flex_space -= consumed;
                                *result = max.get();
                            }
                            else {
                                available_flex_space -= consumed;
                                *result += consumed;
                            }
                            progress_have_been_made = true;
                        }
                    },
                    // for purely flex widgets, 
                    WidgetSizeRequirement::Flex { flex } |
                    WidgetSizeRequirement::Min { flex, .. } => {
                        let consumed = flex_size_it.next_flex(flex);
                        available_flex_space -= consumed;
                        *result += consumed;
                        progress_have_been_made = true;
                    },
                    _ => {},
                }
            }
        }

        results
    }
}


struct DividedAvailableFlexSpace {
    part_size: u32,
    big_part_count: u32,
    small_part_count: u32,
}

impl DividedAvailableFlexSpace {
    fn next_flex(&mut self, flex: NonZeroU32) -> u32 {
        let mut flex_to_take = flex.get();
        let taken_from_big = self.big_part_count.min(flex_to_take);
        self.big_part_count -= taken_from_big;
        flex_to_take -= taken_from_big;
        let taken_from_small = self.small_part_count.min(flex_to_take);
        self.small_part_count -= taken_from_small;
        taken_from_big * (self.part_size + 1) + taken_from_small * self.part_size
    }
}

/// Integer division, splitting the remainer into part of the results.
/// The returned value is in the form ((res1, amount1), (res2, amount2))
/// where the res1 is the quotient of the div, and
/// - res1 * amount1 + res2 * amount2 = divedend,
/// - res1 + 1 = res2
/// - amount1 + amount 2 = divisor
/// This basically performs an integer division, adding the remainder into part of the results
/// This is usefull when splitting spaces onto widgets.
fn full_integer_div(dividend: u32, divisor: NonZeroU32) -> DividedAvailableFlexSpace {
    let quotient = dividend / divisor.get();
    let remainder = dividend % divisor.get();
    DividedAvailableFlexSpace {
        part_size: quotient,
        big_part_count: remainder,
        small_part_count: divisor.get() - remainder,
    }
}



