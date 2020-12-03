import * as guardian from '../data_guardian/src/lib.rs';
(window as any).guardian = guardian;

let guardian_root = (window as any).guardian_root = guardian.guardian_new();

let guardian_a = guardian.guardian_get_map_elem(guardian_root, 1);
guardian.guardian_set_data(guardian_a, 2);

let guardian_b = guardian.guardian_get_map_elem(guardian_root, 3);
guardian.guardian_replace(guardian_b, guardian.guardian_new())

let guardian_ba = guardian.guardian_get_map_elem(guardian_b, 4);
guardian.guardian_set_data(guardian_ba, 5);

let ba_value = guardian.guardian_get_data(
    guardian.guardian_get_map_elem(
        guardian.guardian_get_map_elem(guardian_root, 3),
        4
    )
);
console.log("guardian test " + (ba_value == 5 ? "worked" : "failed"))
