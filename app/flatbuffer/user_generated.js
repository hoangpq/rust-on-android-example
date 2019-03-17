// automatically generated by the FlatBuffers compiler, do not modify

/**
 * @const
 * @namespace
 */
var users = users || {};

/**
 * @constructor
 */
users.User = function() {
  /**
   * @type {flatbuffers.ByteBuffer}
   */
  this.bb = null;

  /**
   * @type {number}
   */
  this.bb_pos = 0;
};

/**
 * @param {number} i
 * @param {flatbuffers.ByteBuffer} bb
 * @returns {users.User}
 */
users.User.prototype.__init = function(i, bb) {
  this.bb_pos = i;
  this.bb = bb;
  return this;
};

/**
 * @param {flatbuffers.ByteBuffer} bb
 * @param {users.User=} obj
 * @returns {users.User}
 */
users.User.getRootAsUser = function(bb, obj) {
  return (obj || new users.User()).__init(
    bb.readInt32(bb.position()) + bb.position(),
    bb
  );
};

/**
 * @param {flatbuffers.Encoding=} optionalEncoding
 * @returns {string|Uint8Array|null}
 */
users.User.prototype.name = function(optionalEncoding) {
  var offset = this.bb.__offset(this.bb_pos, 4);
  return offset
    ? this.bb.__string(this.bb_pos + offset, optionalEncoding)
    : null;
};

/**
 * @param {flatbuffers.Builder} builder
 */
users.User.startUser = function(builder) {
  builder.startObject(1);
};

/**
 * @param {flatbuffers.Builder} builder
 * @param {flatbuffers.Offset} nameOffset
 */
users.User.addName = function(builder, nameOffset) {
  builder.addFieldOffset(0, nameOffset, 0);
};

/**
 * @param {flatbuffers.Builder} builder
 * @returns {flatbuffers.Offset}
 */
users.User.endUser = function(builder) {
  var offset = builder.endObject();
  return offset;
};

/**
 * @param {flatbuffers.Builder} builder
 * @param {flatbuffers.Offset} offset
 */
users.User.finishUserBuffer = function(builder, offset) {
  builder.finish(offset);
};

// Exports for Node.js and RequireJS
this.users = users;
