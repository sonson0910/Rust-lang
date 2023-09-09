#[derive(Debug)]

// fn main() {
//     let hinh_chu_nhat = (30,50);
//     println!("Dien tich cua hinh chu nhat: {}", dien_tich(hinh_chu_nhat));
// }

// fn dien_tich(kichthuoc:(u32, u32)) -> u32{
//     kichthuoc.0 * kichthuoc.1
// }
struct hcn{
    h: u32,
    w: u32,
}

impl hcn{
    fn dien_tich(&self) -> u32{
        self.h * self.w
    }

    fn chua(&self, hcnk: &hcn) -> bool {
        self.h > hcnk.h && self.w > hcnk.w
    }
}

impl hcn{
    fn hinhvuong(kichthuoc: u32) -> hcn{
        hcn { h: (kichthuoc), w: (kichthuoc), }
    }
}

fn main(){
    let kichthuoc = hcn { h: 30, w: 50 };
    println!("Dien tich HCN = {}", kichthuoc.dien_tich());
    println!("Kich thuoc = {:#?}", kichthuoc);

    let kichthuoc2 = hcn{
        h:20,
        w:40,
    };
    let kichthuoc3 = hcn{
        h:40,
        w:60,
    };
    println!("Hinh chu nhat co the chua hinh 2: {}", kichthuoc.chua(&kichthuoc2));
    println!("Hinh chu nhat co the chua hinh 3: {}", kichthuoc.chua(&kichthuoc3));
}

// fn dien_tich(kichthuoc: &hcn) -> u32{
//     kichthuoc.h * kichthuoc.w
// }