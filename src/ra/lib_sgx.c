#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sgx_uae_epid.h>

/**
unsigned char platform_info[101];
[4, 0, 1, 0, 0, 10, 10, 2, 2, 255, 1, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 12, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 12, 43, 203, 17, 175, 217, 219, 123, 197, 214, 111, 131, 120, 182, 40, 242, 244, 174, 175, 26, 80, 212, 110, 171, 231, 243, 42, 34, 212, 237, 223, 25, 192, 151, 96, 9, 88, 251, 57, 153, 135, 80, 192, 125, 152, 142, 247, 140, 235, 42, 147, 93, 29, 212, 232, 32, 135, 219, 150, 2, 163, 110, 40, 114, 163, 3]
*/
int lib_c_sgx_report_att_status(unsigned char platform_info[]) {
  sgx_status_t status;
  sgx_update_info_bit_t update_info;

  status = sgx_report_attestation_status(
                (sgx_platform_info_t*)platform_info,
                1,
                &update_info);

  switch (status) {
    case SGX_SUCCESS:
      printf("SGX_SUCCESS\n");
      break;
    case SGX_ERROR_INVALID_PARAMETER:
      printf("SGX_ERROR_INVALID_PARAMETER\n");
      break;
    case SGX_ERROR_AE_INVALID_EPIDBLOB:
      printf("SGX_ERROR_AE_INVALID_EPIDBLOB\n");
      break;
    case SGX_ERROR_UPDATE_NEEDED:
      printf("SGX_ERROR_UPDATE_NEEDED\n");
      printf("ucodeUpdate = %d\n", update_info.ucodeUpdate);
      printf("csmeFwUpdate = %d\n", update_info.csmeFwUpdate);
      printf("pswUpdate = %d\n", update_info.pswUpdate);
      break;
    case SGX_ERROR_OUT_OF_MEMORY:
      printf("SGX_ERROR_OUT_OF_MEMORY\n");
      break;
    case SGX_ERROR_SERVICE_UNAVAILABLE:
      printf("SGX_ERROR_SERVICE_UNAVAILABLE\n");
      break;
    case SGX_ERROR_SERVICE_TIMEOUT:
      printf("SGX_ERROR_SERVICE_TIMEOUT\n");
      break;
    case SGX_ERROR_BUSY:
      printf("SGX_ERROR_BUSY\n");
      break;
    case SGX_ERROR_NETWORK_FAILURE:
      printf("SGX_ERROR_NETWORK_FAILURE\n");
      break;
    case SGX_ERROR_OUT_OF_EPC:
      printf("SGX_ERROR_OUT_OF_EPC\n");
      break;
    case SGX_ERROR_UNRECOGNIZED_PLATFORM:
      printf("SGX_ERROR_UNRECOGNIZED_PLATFORM\n");
      break;
    case SGX_ERROR_UNEXPECTED:
      printf("SGX_ERROR_UNEXPECTED\n");
      break;
    default:
      printf("----------------------------------------------\n");
      break;
  }
  return 0;
}