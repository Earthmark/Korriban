use cgmath::{Deg, Matrix4, Quaternion, Rotation3, Vector3, vec3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use korriban::prop::{PropSet};
use num_traits::identities::Zero;

fn props_access(c: &mut Criterion) {

  let mut set = PropSet::new();
  let key2 = set.allocate::<i32>(123);
  let key = set.allocate::<Vector3<i32>>(Vector3 { x: 1, y: 2, z: 3 });

  c.bench_function("props rw", |b| b.iter(|| {
        *set.get_mut(&key2).unwrap() = black_box(2);
        (
            *set.get(black_box(&key2)).unwrap(),
            *set.get(black_box(&key2)).unwrap(),
            *set.get(black_box(&key)).unwrap(),
            *set.get(black_box(&key)).unwrap(),
            *set.get(black_box(&key)).unwrap(),
        )
  }));
}

fn props_multi_sum(c: &mut Criterion) {

  let mut set = PropSet::new();
  let t = set.allocate::<Vector3<f32>>(vec3(10.0, 5.0, 2.0));
  let r = set.allocate::<Quaternion<f32>>(Rotation3::from_angle_y(Deg(45.0)));
  let s = set.allocate::<f32>(2.0);
  let m = set.allocate::<Matrix4<f32>>(Matrix4::zero());

  c.bench_function("trs create", |b| b.iter(|| {
    let vt = set.get(black_box(&t)).unwrap();
    let vr = set.get(black_box(&r)).unwrap();
    let vs = set.get(black_box(&s)).unwrap();

    *set.get_mut(black_box(&m)).unwrap() = Matrix4::from_translation(*vt) * Matrix4::from(*vr) * Matrix4::from_scale(*vs) 
  }));
}

criterion_group!(props_benches, props_access, props_multi_sum);
criterion_main!(props_benches);
