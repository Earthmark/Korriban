use glam::{Mat4, Quat, Vec3, IVec3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use korriban::prop::{PropSet};

fn props_access(c: &mut Criterion) {
  let mut set = PropSet::new();
  let key2 = set.allocate::<i32>(123);
  let key = set.allocate::<IVec3>(IVec3::new(1, 2, 3 ));

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
  let t = set.allocate::<Vec3>(Vec3::new(10.0, 5.0, 2.0));
  let r = set.allocate::<Quat>(Quat::from_rotation_y(f32::to_radians(45.0)));
  let s = set.allocate::<Vec3>(Vec3::new(2.0, 2.0, 2.0));
  let m = set.allocate::<Mat4>(Mat4::ZERO);

  c.bench_function("trs create", |b| b.iter(|| {
    let vt = set.get(black_box(&t)).unwrap();
    let vr = set.get(black_box(&r)).unwrap();
    let vs = set.get(black_box(&s)).unwrap();

    *set.get_mut(black_box(&m)).unwrap() = Mat4::from_scale_rotation_translation(*vs, *vr, *vt)
  }));
}

criterion_group!(props_benches, props_access, props_multi_sum);
criterion_main!(props_benches);
