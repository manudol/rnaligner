#ifndef VIENNARNA_H
#define VIENNARNA_H

#include <ViennaRNA/utils/basic.h>
#include <ViennaRNA/fold_compound.h>
#include <ViennaRNA/mfe.h>

char* vienna_fold(char* seq);
void free_vienna_fold_result(void *ptr);

#endif

