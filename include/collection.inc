//====================================================
//				SA-MP collection plugin
//			Rust std::collections for SA-MP
//
//				version: v0.0.1 alpha
//
//		Copyright(c) 2019 Qoo. All rights reserved.
//====================================================

#if defined __COLLECTION_H__
	#endinput
#endif
#define __COLLECTION_H__

#pragma library collection

enum _: e_VALUE_TYPE
{
	VALUE_TYPE_INTEGER = 1,
	VALUE_TYPE_FLOAT = 2,
	VALUE_TYPE_ARRAY = 3
};

#include <collection_vec>
