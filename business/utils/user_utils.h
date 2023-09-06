/*********************************************************************
 * Copyright (c) keyst.one. 2020-2025. All rights reserved.
 * name       : user_utils.h
 * Description:
 * author     : stone wang
 * data       : 2022-12-29 16:40
**********************************************************************/

#ifndef _USER_UTILS_H
#define _USER_UTILS_H

#include <ctype.h>
#include "stdint.h"
#include "stdbool.h"
#include "string.h"

#define CLEAR_ARRAY(array)                      memset(array, 0, sizeof(array))
#define CLEAR_OBJECT(obj)                       memset(&obj, 0, sizeof(obj))
#define VALUE_CHECK(value, expect)              {if (value != expect) {printf("input err!\r\n"); return; }}


uint32_t StrToHex(uint8_t *pbDest, const char *pbSrc);
void ByteArrayToHexStr(uint8_t *array, uint32_t len, char *hex);
bool CheckEntropy(const uint8_t *array, uint32_t len);
bool CheckAllFF(const uint8_t *array, uint32_t len);
bool CheckAllZero(const uint8_t *array, uint32_t len);
void RemoveFormatChar(char *str);
void ArrayRandom(char *words, char *out, int count);
int WordsListSlice(char *words, char wordsList[][10], uint8_t wordsCount);

#endif /* _USER_UTILS_H */
