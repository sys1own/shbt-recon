/* shbt_cryo_kernel.c — Debye T^3 integrated energy density for the CVD
 * diamond substrate, and NbN quench headroom verification.
 *
 *   u(T) = (3 pi^4 n R / (5 Theta_D^3)) T^4,  n = rho / M_C (mol/m^3)
 */
#include "shbt_cryo_kernel.h"
#include <math.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

static inline double molar_density(void) {
    return RHO_DIAMOND / MOLAR_MASS_C;
}

double calculate_diamond_volumetric_energy(double t_k) {
    double n_molar = molar_density();
    double coeff = (3.0 * M_PI * M_PI * M_PI * M_PI * n_molar * R_GAS)
                   / (5.0 * pow(THETA_D, 3.0));
    return coeff * pow(t_k, 4.0);
}

int verify_cryogenic_quench_headroom(double t_peak, double *headroom_out) {
    double u_peak = calculate_diamond_volumetric_energy(t_peak);
    *headroom_out = 16.0 - t_peak;
    if (*headroom_out < 10.0 || u_peak > 10.0) {
        return -1;
    }
    return 0;
}
