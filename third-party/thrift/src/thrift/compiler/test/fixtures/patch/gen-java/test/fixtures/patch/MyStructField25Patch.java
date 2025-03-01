/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.patch;

import com.facebook.swift.codec.*;
import com.facebook.swift.codec.ThriftField.Requiredness;
import com.facebook.swift.codec.ThriftField.Recursiveness;
import com.google.common.collect.*;
import java.util.*;
import javax.annotation.Nullable;
import org.apache.thrift.*;
import org.apache.thrift.transport.*;
import org.apache.thrift.protocol.*;
import static com.google.common.base.MoreObjects.toStringHelper;
import static com.google.common.base.MoreObjects.ToStringHelper;

@SwiftGenerated
@com.facebook.swift.codec.ThriftStruct(value="MyStructField25Patch", builder=MyStructField25Patch.Builder.class)
public final class MyStructField25Patch implements com.facebook.thrift.payload.ThriftSerializable {

    @ThriftConstructor
    public MyStructField25Patch(
        @com.facebook.swift.codec.ThriftField(value=1, name="assign", requiredness=Requiredness.OPTIONAL) final test.fixtures.patch.MyEnum assign,
        @com.facebook.swift.codec.ThriftField(value=2, name="clear", requiredness=Requiredness.NONE) final boolean clear
    ) {
        this.assign = assign;
        this.clear = clear;
    }
    
    @ThriftConstructor
    protected MyStructField25Patch() {
      this.assign = null;
      this.clear = false;
    }
    
    public static class Builder {
    
        private test.fixtures.patch.MyEnum assign = null;
        private boolean clear = false;
    
        @com.facebook.swift.codec.ThriftField(value=1, name="assign", requiredness=Requiredness.OPTIONAL)
        public Builder setAssign(test.fixtures.patch.MyEnum assign) {
            this.assign = assign;
            return this;
        }
    
        public test.fixtures.patch.MyEnum getAssign() { return assign; }
    
            @com.facebook.swift.codec.ThriftField(value=2, name="clear", requiredness=Requiredness.NONE)
        public Builder setClear(boolean clear) {
            this.clear = clear;
            return this;
        }
    
        public boolean isClear() { return clear; }
    
        public Builder() { }
        public Builder(MyStructField25Patch other) {
            this.assign = other.assign;
            this.clear = other.clear;
        }
    
        @ThriftConstructor
        public MyStructField25Patch build() {
            MyStructField25Patch result = new MyStructField25Patch (
                this.assign,
                this.clear
            );
            return result;
        }
    }
    
    public static final Map<String, Integer> NAMES_TO_IDS = new HashMap();
    public static final Map<String, Integer> THRIFT_NAMES_TO_IDS = new HashMap();
    public static final Map<Integer, TField> FIELD_METADATA = new HashMap<>();
    private static final TStruct STRUCT_DESC = new TStruct("MyStructField25Patch");
    private final test.fixtures.patch.MyEnum assign;
    public static final int _ASSIGN = 1;
    private static final TField ASSIGN_FIELD_DESC = new TField("assign", TType.I32, (short)1);
        private final boolean clear;
    public static final int _CLEAR = 2;
    private static final TField CLEAR_FIELD_DESC = new TField("clear", TType.BOOL, (short)2);
    static {
      NAMES_TO_IDS.put("assign", 1);
      THRIFT_NAMES_TO_IDS.put("assign", 1);
      FIELD_METADATA.put(1, ASSIGN_FIELD_DESC);
      NAMES_TO_IDS.put("clear", 2);
      THRIFT_NAMES_TO_IDS.put("clear", 2);
      FIELD_METADATA.put(2, CLEAR_FIELD_DESC);
      com.facebook.thrift.type.TypeRegistry.add(new com.facebook.thrift.type.Type(
        new com.facebook.thrift.type.UniversalName("test.dev/fixtures/patch/MyStructField25Patch"), 
        MyStructField25Patch.class, MyStructField25Patch::read0));
    }
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=1, name="assign", requiredness=Requiredness.OPTIONAL)
    public test.fixtures.patch.MyEnum getAssign() { return assign; }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=2, name="clear", requiredness=Requiredness.NONE)
    public boolean isClear() { return clear; }
    
    @java.lang.Override
    public String toString() {
        ToStringHelper helper = toStringHelper(this);
        helper.add("assign", assign);
        helper.add("clear", clear);
        return helper.toString();
    }
    
    @java.lang.Override
    public boolean equals(java.lang.Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
    
        MyStructField25Patch other = (MyStructField25Patch)o;
    
        return
            Objects.equals(assign, other.assign) &&
            Objects.equals(clear, other.clear) &&
            true;
    }
    
    @java.lang.Override
    public int hashCode() {
        return Arrays.deepHashCode(new java.lang.Object[] {
            assign,
            clear
        });
    }
    
    
    public static com.facebook.thrift.payload.Reader<MyStructField25Patch> asReader() {
      return MyStructField25Patch::read0;
    }
    
    public static MyStructField25Patch read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin(MyStructField25Patch.NAMES_TO_IDS, MyStructField25Patch.THRIFT_NAMES_TO_IDS, MyStructField25Patch.FIELD_METADATA);
      MyStructField25Patch.Builder builder = new MyStructField25Patch.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        case _ASSIGN:
          if (__field.type == TType.I32) {
            test.fixtures.patch.MyEnum assign = test.fixtures.patch.MyEnum.fromInteger(oprot.readI32());
            builder.setAssign(assign);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _CLEAR:
          if (__field.type == TType.BOOL) {
            boolean clear = oprot.readBool();
            builder.setClear(clear);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
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
      if (assign != null) {
        oprot.writeFieldBegin(ASSIGN_FIELD_DESC);
        oprot.writeI32(this.assign == null ? 0 : this.assign.getValue());
        oprot.writeFieldEnd();
      }
      oprot.writeFieldBegin(CLEAR_FIELD_DESC);
      oprot.writeBool(this.clear);
      oprot.writeFieldEnd();
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
    private static class _MyStructField25PatchLazy {
        private static final MyStructField25Patch _DEFAULT = new MyStructField25Patch.Builder().build();
    }
    
    public static MyStructField25Patch defaultInstance() {
        return  _MyStructField25PatchLazy._DEFAULT;
    }
}
