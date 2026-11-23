# MISB ST 1201.5 — IMAPB (floating point to integer mapping), §7 extraction

> **Source.** MISB ST 1201.5 "Floating Point to Integer Mapping", Motion Imagery
> Standards Board, 24 June 2021, 25 pages. Vendored PDF:
> `specs/misb_st1201.5_2021_floating_point_to_integer_mapping.pdf`
> (SHA-256 in the per-file PROVENANCE note beside it). Free US Government
> publication from the NGA/NSG registry; obtained 2026-11-22 from the same public
> mirror route as ST 0601.19.
>
> **Scope.** Only the IMAPB algorithm ST 0601's IMAPB items delegate to:
> §7.1.2 (Starting Point B — specifying length), §7.2 (forward and reverse
> mapping) and §7.2.3 (special values, Tables 1–3). Everything else in ST 1201.5
> (IMAPA/Starting Point A, worked derivations, §8) is out of scope for the
> transcribed table.
>
> **How it was produced.** `cd ~/Projects/pdf2md && uv run pdf2md convert <pdf> -o <md>
> --engine hybrid --pages 5-9 --report` (exit 0, no verifier warnings); table cells
> the converter split were checked against the PDF text layer. The text below is
> the converter's output, verbatim apart from normalised heading levels.
>
> **Why it is here.** ST 0601 §8 delegates every `IMAPB(min,max,length)` item to
> this algorithm, so a scaling test cannot check those items against ST 0601
> alone. `st0601-tag-table.md` names the IMAPB bounds per item; the formula is
> this file's.

---

7.1.2 Starting Point B : Specifying Length

Input

• a = Floating-Point Minimum, range is from smallest float value to largest float value

| • | b = Floating-Point Maximum, range is from smallest float value to largest float value |
|---|---|
| • | L = Length of resulting mapped integer (note : this includes an extra bit for the special values, |
|  | see below) |
| • | Note : a<b |

Algorithm Compute constants for forward and reverse mapping

1. bPow = ceiling(log2(b-a))

2. dPow = 8*L-1

3. sF = 2^( dPow -bPow)

4. sR = 2^(bPow- dPow)

5. Zoffset = 0.0

6. if ( a<0 and b>0) then Zoffset = sF*a-floor(sF*a)

24 June 2021 Motion Imagery Standards Board 5

7.2 Step 2 : Forward and Reverse Mapping

Step 1 computes the scaling factors for the forward and reverse mapping in Step 2. These

operations are computationally optimal, so the mapping requires only one multiply and two adds. Because the multiplication uses power of two values, an option is to implement the

multiplication using floating point binary shifts (i.e., in hardware constrained environments).

7.2.1 Forward Mapping

Input

• sF = Forward scaling factor ( computed in Step 1)

• a = Floating-Point Minimum

• b = Floating-Point Maximum

• Zoffset = Zero-point offset ( computed in Step 1)

• x = floating-point value to map to integer y

Algorithm Compute mapped integer value y

1. If x is a special value, then :

y = Table Lookup to map to specified bit pattern ( see Section 7.2.3)

2. If x<a then :

y = Special bit pattern IMAP_BELOW_MINIMUM ( see Section 7.2.3.1)

3. If x>b then :

y = Special bit pattern IMAP_ABOVE_MAXIMUM ( see Section 7.2.3.1)

4. Else x is a normal floating-point number then :

a. y=truncate(sF*(x-a)+Zoffset)

i. Note : Since sF is a power of 2, an option is to use a floating-point shift instead

of a multiplication

ii. Note : Convert y into L number of bytes values (i.e., the lower L number of

bytes is the “value” to send)

Developer Notes

1) IEEE 754 floating-point numbers do not explicitly represent the most significant 1-bit in the

mantissa, so if performing a manual shift (i.e., the CPU does not support a floating-point shift), prepend a leading bit to the mantissa before shifting and converting to an integer. As a simple example, the IEEE 754 mantissa for 6 is 1000..., it first becomes 11000..., then...00000110. 2) If special values are not in use for a value, an optimization of the forward mapping is to skip the “special values check” and omit the table look-up. Preserve the extra bit ( special value bit) in the data format but remove the conditional check ( Step 1 above).

24 June 2021 Motion Imagery Standards Board 6

7.2.2 Reverse Mapping

Input

• sR = Reverse scaling factor ( computed in Step 1)

• a = Floating-Point Minimum

• Zoffset = Zero-point offset ( computed in Step 1)

• y = integer value to map to floating-point value x

Algorithm Compute mapped floating-point value x

Define : Bit(q,y) = the qth bit of y. msb = Most Significant Bit

1. special_value = Bit(msb,y) & Bit(msb-1,y)

2. If special_value equals 1 then

a. x=table lookup to map to specified floating-point value ( Section see 7.2.3 Table 2

below)

3. Else y is a normal value then

a. x=sR*(y-Zoffset) + a

i. Note : Since sR is a power of 2, an option is to use a floating-point shift instead

of a multiplication

Developer Notes

1) IEEE 754 floating-point numbers do not explicitly represent the most significant 1-bit in the

mantissa, so if performing a manual shift (i.e., the CPU does not support a floating-point shift), prepend a leading bit to the mantissa before shifting and converting to an integer. As a

simple example, the mantissa for 6 is 1000..., it first becomes 11000..., then...00000110. 2) Before applying this algorithm, verify the length is the correct length for the pre-computed sR value, see Section 7.4 for further information.

7.2.3 Special Value Mappings

In addition to normal numeric values this standard defines a list of special values to indicate

unusual circumstances, such as a gimbal lock, division by zero, etc. This standard defines existing standardized values ( based on IEEE-754) and a set of MISB definable values.

The first two bits indicate either a normal or special value as shown in Table 1. The msb is zero

for all normal mapped values except for the maximum floating-point value, b, but only if (b-a),

as specified by the user, is a power of 2.

24 June 2021 Motion Imagery Standards Board 7

Table 1 : Bit Patterns

Bits

| b n | bn-1 bn-2 | – b0 | Description |
|---|---|---|---|
| (msb) |  |  |  |
| 0 | x | x Normal | mapped value ( x=1 or 0) |

Normal mapped value. This is the max value of the normal mapping 1 0 0 values; this is the only normal value with msb=1.

1 0 x ( At least one x bit = 1) – Reserved section of bit space

1 1 x Special value indicator ( x=1 or 0) – see Table 2

If both the first and second msb are set to one, then the third, fourth and fifth msbs indicate

which special value it is, as shown in Table 2. Since the mapped integer has fewer bytes than the original floating-point number not all the source NaN Identifiers can be defined. The default

NaN Identifier is a value with all zeros.

Table 2 : Special Value Bit Patterns

| Name | Most | Significant | Bits | Other | Description - IEEE-754 | values |
|---|---|---|---|---|---|---|
|  |  |  |  | bits | ( or user defined |) |
|  | bn bn-1 | bn-2 | bn-3 | bn-4 bn-5 – b0 |  |  |
|  | (msb) (Special) | (Sign) | (NaN) |  |  |  |
|  |  |  |  | Zero |  |  |
| +Infinity | 1 1 | 0 | 0 | 1 | Positive Infinity (+∞) |  |
|  |  |  |  | Filled |  |  |
|  |  |  |  | Zero |  |  |
| -Infinity | 1 1 | 1 | 0 | 1 | Negative Infinity (-∞) |  |
|  |  |  |  | Filled |  |  |
| +QNaN | 1 1 | 0 | 1 | 0 NaN Id* | Positive Quiet NaN ( Not a | Number) |
| -QNaN | 1 1 | 1 | 1 | 0 NaN Id* | Negative Quiet NaN |  |

Positive Signal NaN – “Other bits” are the +SNaN 1 1 0 1 1 NaN Id* signal value

Negative Signal NaN – “Other bits” are the

| -SNaN | 1 | 1 | 1 | 1 | 1 | NaN Id* |
|---|---|---|---|---|---|---|
|  |  |  |  |  |  | signal value |
| MISB |  |  |  |  |  | MISB MISB Defined Special Values. |
|  | 1 | 1 | 1 | 0 | 0 |  |
| Defined |  |  |  |  |  | Defined See Section 7.2.3.1 |
| User |  |  |  |  |  | User |
|  | 1 | 1 | 0 | 0 | 0 | “Other bits” enumerate user defined signals |
| Defined |  |  |  |  |  | Defined |
| Reserved | 1 | 0 | X | X | X | X Reserved ( At least one “X” is set to 1) |

*This standard defines only one NaN Identifier, i.e., fill bits bn-5 – b0 with zeros. The MISB will define NaN Identifier’s as necessary; please contact the MISB to define new values. The values need to be acceptable universally and not special values for a specific processor, application or

program.

24 June 2021 Motion Imagery Standards Board 8

This standard does not define a special value for the IEEE 754 negative zero value; use positive

zero to represent negative zero values.

Requirement ST 1201.1-12 All negative zero values shall be mapped to a positive zero and sent as a normal (non-special) value.

In addition to the IEEE bit values a set of MISB-defined bit patterns are available for use within

a Parent Document. For example, in MISB ST 0601 [2] the sensor depression angle value could include a bit pattern to indicate gimbal lock; this bit pattern would then only be valid for that

value in ST 0601.

Developers note : Future versions of this standard may use one or more of the Reserved special

values in Table 2. For forward-compatibility (i.e., reverse map a value which was forward mapped with newer standard implementation), implementations need to handle these “unknown”

special cases (e.g., raise exception, set flags, etc.) preventing a software failure.

7.2.3.1 MISB Defined Special Values

The MISB defines special values for signaling information when IMAP encodes values, e.g., an IMAP overflow. Currently, the defined and RESERVED values need only one byte. Future expansions (i.e., RESERVED-GROWTH) will add bytes as needed. Table 3 lists the MISB

Defined Special Values.

Table 3 : MISB Defined Special Values

| Name | Most | Significant | Bits | MISB Special | Other Bits |
|---|---|---|---|---|---|
|  | ( From | Table | 2) | Value Bits |  |
|  | bn bn-1 | bn-2 | bn-3 | bn-4 bn-5 bn-6 | bn-7 bn-8 – b0 |
|  | (msb) (Special) | (Sign) | (NaN) |  |  |
| IMAP_BELOW_MINIMUM | 1 1 | 1 | 0 | 0 0 0 | 0 Zero Filled |
| IMAP_ABOVE_MAXIMUM | 1 1 | 1 | 0 | 0 0 0 | 1 Zero Filled |
| RESERVED | 1 1 | 1 | 0 | 0 0 1 | 0 Zero Filled |
| RESERVED | 1 1 | 1 | 0 | 0 0 1 | 1 Zero Filled |
| RESERVED | 1 1 | 1 | 0 | 0 1 0 | 0 Zero Filled |
| RESERVED | 1 1 | 1 | 0 | 0 1 0 | 1 Zero Filled |
| RESERVED | 1 1 | 1 | 0 | 0 1 1 | 0 Zero Filled |
| RESERVED - GROWTH | 1 1 | 1 | 0 | 0 1 1 | 1 RESERVED |

The IMAP_BELOW_MINIMUM special value signals an IMAP encoding utilized a value less than the minimum value of the IMAP range ( see Section 7.2.1 for how this applies in the 1 encoding algorithm). For example, using IMAPB (-50.0, 5000.0, 3) to encode the value -100.0

produces the IMAP_BELOW_MINIMUM value.

1 See Section 7.3 for description of IMAPB notation.

24 June 2021 Motion Imagery Standards Board 9