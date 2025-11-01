#include <stdio.h>
#include <string.h>
#include <stdlib.h>


#include <ViennaRNA/utils/basic.h>
#include <ViennaRNA/fold_compound.h>
#include <ViennaRNA/mfe.h>


char* vienna_fold(char* seq)
{
    char *ss;
    float mfe;
    vrna_fold_compound_t *fc;
    
    ss = vrna_alloc(sizeof(char) * (strlen(seq) + 1));
    fc = vrna_fold_compound(seq, NULL, VRNA_OPTION_DEFAULT);
    mfe = vrna_mfe(fc, ss);
    vrna_fold_compound_free(fc); 
    return ss;
}

void free_vienna_fold_result(void *ptr)
{
    if (ptr != NULL) {
        free(ptr);
    }
}
