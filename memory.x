/* See section 4 "Memory mapping" of the datasheet */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2M
  /* Reduced to work around https://github.com/probe-rs/probe-rs/issues/429 */
  RAM : ORIGIN = 0x24000000, LENGTH = 512K
}
