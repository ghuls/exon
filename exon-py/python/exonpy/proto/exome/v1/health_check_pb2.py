# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: exome/v1/health_check.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1b\x65xome/v1/health_check.proto\x12\x08\x65xome.v1\"%\n\x12HealthCheckRequest\x12\x0f\n\x07service\x18\x01 \x01(\t\"\xa3\x01\n\x13HealthCheckResponse\x12;\n\x06status\x18\x01 \x01(\x0e\x32+.exome.v1.HealthCheckResponse.ServingStatus\"O\n\rServingStatus\x12\x0b\n\x07UNKNOWN\x10\x00\x12\x0b\n\x07SERVING\x10\x01\x12\x0f\n\x0bNOT_SERVING\x10\x02\x12\x13\n\x0fSERVICE_UNKNOWN\x10\x03\x32P\n\x06Health\x12\x46\n\x05\x43heck\x12\x1c.exome.v1.HealthCheckRequest\x1a\x1d.exome.v1.HealthCheckResponse\"\x00\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'exome.v1.health_check_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_HEALTHCHECKREQUEST']._serialized_start=41
  _globals['_HEALTHCHECKREQUEST']._serialized_end=78
  _globals['_HEALTHCHECKRESPONSE']._serialized_start=81
  _globals['_HEALTHCHECKRESPONSE']._serialized_end=244
  _globals['_HEALTHCHECKRESPONSE_SERVINGSTATUS']._serialized_start=165
  _globals['_HEALTHCHECKRESPONSE_SERVINGSTATUS']._serialized_end=244
  _globals['_HEALTH']._serialized_start=246
  _globals['_HEALTH']._serialized_end=326
# @@protoc_insertion_point(module_scope)
