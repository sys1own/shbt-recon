/* shbt_cryo_kernel.h — CVD diamond Debye T^3 cryogenic headroom integrator. */
#ifndef SHBT_CRYO_KERNEL_H
#define SHBT_CRYO_KERNEL_H

#define R_GAS        8.314462618
#define THETA_D      2230.0
#define RHO_DIAMOND  3515.0
#define MOLAR_MASS_C 0.012011

double calculate_diamond_volumetric_energy(double t_k);
int verify_cryogenic_quench_headroom(double t_peak, double *headroom_out);

#endif /* SHBT_CRYO_KERNEL_H */
