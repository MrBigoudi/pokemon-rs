#[derive(Debug)]
pub enum ExperienceGroup {
    Erratic,
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
    Fluctuating,
}

pub fn get_experience_from_group(group: ExperienceGroup, level: u8) -> u32 {
    if level == 100 {
        return 0;
    }

    let n = level as f64;
    let n_2 = n * n;
    let n_3 = n_2 * n;
    match group {
        ExperienceGroup::Erratic => {
            if n < 50. {
                ((n_3 * (100. - n)) / 50.) as u32
            } else if n < 68. {
                ((n_3 * (150. - n)) / 100.) as u32
            } else if n < 98. {
                ((n_3 * ((1911. - 10. * n) / 3.).floor()) / 500.) as u32
            } else {
                ((n_3 * (160. - n)) / 100.) as u32
            }
        }
        ExperienceGroup::Fast => (0.8 * n_3) as u32,
        ExperienceGroup::MediumFast => n_3 as u32,
        ExperienceGroup::MediumSlow => (1.2 * n_3 - 15. * n_2 + 100. * n - 140.) as u32,
        ExperienceGroup::Slow => (1.25 * n_3) as u32,
        ExperienceGroup::Fluctuating => {
            if n < 15. {
                ((n_3 * (((n + 1.) / 3.).floor() + 24.)) / 50.) as u32
            } else if n < 36. {
                ((n_3 * (n + 14.)) / 50.) as u32
            } else {
                ((n_3 * ((n / 2.).floor() + 32.)) / 50.) as u32
            }
        }
    }
}
