#[derive(Clone, Debug, Copy, strum::Display, strum::EnumString)]
pub enum Consonants {
    K,
    Ky,
    G,
    Gy,
    S,
    Sy,
    Sh,
    Sg,
    Z,
    J,
    Zy,
    Zg,
    T,
    Ty,
    Th,
    D,
    Dy,
    Dh,
    N,
    Ny,
    H,
    Hy,
    Hg,
    F,
    Fy,
    B,
    By,
    Bg,
    P,
    Py,
    Pg,
    M,
    My,
    Y,
    R,
    Ry,
    W,
    Wh,
    V,
}

impl Consonants {
    pub fn new() -> std::vec::Vec<Consonants> {
        let mut out: Vec<Consonants> = Vec::with_capacity(39);
        out.extend(vec![
            Consonants::K,
            Consonants::Ky,
            Consonants::G,
            Consonants::Gy,
            Consonants::S,
            Consonants::Sy,
            Consonants::Sh,
            Consonants::Sg,
            Consonants::Z,
            Consonants::J,
            Consonants::Zy,
            Consonants::Zg,
            Consonants::T,
            Consonants::Ty,
            Consonants::Th,
            Consonants::D,
            Consonants::Dy,
            Consonants::Dh,
            Consonants::N,
            Consonants::Ny,
            Consonants::H,
            Consonants::Hy,
            Consonants::Hg,
            Consonants::F,
            Consonants::Fy,
            Consonants::B,
            Consonants::By,
            Consonants::Bg,
            Consonants::P,
            Consonants::Py,
            Consonants::Pg,
            Consonants::M,
            Consonants::My,
            Consonants::Y,
            Consonants::R,
            Consonants::Ry,
            Consonants::W,
            Consonants::Wh,
            Consonants::V,
        ]);
        out
    }
}
