/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.stream;

import com.facebook.swift.codec.*;
import com.facebook.swift.codec.ThriftField.Requiredness;
import com.facebook.swift.codec.ThriftField.Recursiveness;
import java.util.*;
import javax.annotation.Nullable;
import org.apache.thrift.*;
import org.apache.thrift.transport.*;
import org.apache.thrift.protocol.*;

@SwiftGenerated
@com.facebook.swift.codec.ThriftStruct("FooEx2")
public final class FooEx2 extends org.apache.thrift.TBaseException implements com.facebook.thrift.payload.ThriftSerializable {
    private static final long serialVersionUID = 1L;


    public static final Map<String, Integer> NAMES_TO_IDS = new HashMap();
    public static final Map<String, Integer> THRIFT_NAMES_TO_IDS = new HashMap();
    public static final Map<Integer, TField> FIELD_METADATA = new HashMap<>();

    private static final TStruct STRUCT_DESC = new TStruct("FooEx2");

    @ThriftConstructor
    public FooEx2(
    ) {
    }
    
    
    
    public static class Builder {
    
    
    public Builder() { }
        public Builder(FooEx2 other) {
        }
    
        @ThriftConstructor
        public FooEx2 build() {
            FooEx2 result = new FooEx2 (
            );
            return result;
        }
    }
    

    
    public static com.facebook.thrift.payload.Reader<FooEx2> asReader() {
      return FooEx2::read0;
    }
    
    public static FooEx2 read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin(FooEx2.NAMES_TO_IDS, FooEx2.THRIFT_NAMES_TO_IDS, FooEx2.FIELD_METADATA);
      FooEx2.Builder builder = new FooEx2.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        default:
          TProtocolUtil.skip(oprot, __field.type);
          break;
        }
        oprot.readFieldEnd();
      }
      oprot.readStructEnd();
      return builder.build();
    }
    
    public void write0(TProtocol oprot) throws TException {
      oprot.writeStructBegin(STRUCT_DESC);
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
    private static class _FooEx2Lazy {
        private static final FooEx2 _DEFAULT = new FooEx2.Builder().build();
    }
    
    public static FooEx2 defaultInstance() {
        return  _FooEx2Lazy._DEFAULT;
    }}
