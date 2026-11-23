MISB ST 1607

STANDARD

Constructs to Amend/Segment KLV Metadata 27 October 2016

# 1 Scope

This standard defines two KLV Local Set constructs to amend and segment KLV metadata.

These constructs are categorized as Local Sets (LS) per the definitions in SMPTE ST 336 [1].

A metadata life cycle begins at the creation of metadata, which is called first-generation metadata. This life cycle includes adding additional metadata, editing existing metadata and deleting metadata in subsequent processing. One of the goals of the MISP is to retain all metadata – both first-generation metadata and subsequent changes in metadata, for analysis and legal purposes.

The standard defines the Amend Local Set for editing metadata using a Tree structure of parent/child relationships, while preserving first-generation metadata. The Amend Local Set enables child or branch metadata, residing within the first-generation metadata’s set (the parent or root metadata set), and provides tight cohesion between first-generation and changed metadata. The Amend Local Set does not have predefined member metadata items; its members are “inherited” from the parent metadata set’s definition (e.g. a Local Set’s Standard) based on application needs.

This standard also defines the Segment Local Set for distributing metadata, such that metadata is distributed between an instantiating parent set, and one or more child sets. The Segment Local Set does not have predefined member metadata items; its members are “inherited” from the parent metadata set’s definition based on application needs.

The standard also defines requirements for using the Amend Local Set and Segment Local Set within a root metadata set.

# 2 References

[1] SMPTE ST 336:2007 Data Encoding Protocol Using Key-Length-Value.

[2] MISB ST 0807.18 MISB KLV Metadata Registry, Oct 2016.

[3] MISB MISP-2017.1: Motion Imagery Handbook, Oct 2016.

[4] MISB ST 0102.11 Security Metadata Universal and Local Sets for Digital Motion Imagery, Oct 2014.

[5] MISB MISP-2017.1 Motion Imagery Standards Profile, Oct 2016.

# 3 Acronyms

|  |  |
|---|---|
| ID | Identifier |
| KLV | Key-Length-Value |
| LS | Local Set |
| MISB | Motion Imagery Standards Board |
| MISP | Motion Imagery Standards Profile |

# 4 Terms

Element – used here to denote a SMPTE ST 336 data item or data group.

First-generation metadata – the birthing of metadata at its source. Constitutes the root level metadata within a metadata tree hierarchy.

# 5 Revision History

| Revision | Date | Summary of Changes |
|---|---|---|
| ST 1607 | 10/27/2016 | Initial Release |

# 6 Introduction

New user applications require new metadata elements to be defined. To foster interoperability, these new metadata elements are created and registered in the MISB KLV Metadata Registry [2]. Beyond adding new metadata elements, however, is the need to extend metadata, such as reusing one or more metadata elements per instantiation of a metadata set, and to edit existing metadata while preserving first-generation metadata.

The Motion Imagery Handbook [3] Section 7.1 introduces the notion of a metadata life cycle, where data preservation and data sharing are key concepts in the management of metadata. As introduced in the handbook, the enhanced operations use “Amend Metadata”, which enables existing metadata to be modified, while preserving original data, and “Segment Metadata”, which enables sharing metadata elements, while other metadata elements are partitioned to describe related but different information within Motion Imagery. Both operations meet the goals of data preservation and data sharing. Tree structures can be built using Amend Metadata and Segment Metadata to form parent/child relationships, where the top-most parent is the “root” and a child is a “branch” of the tree.

For implementation, the “Amend Local Set” (i.e. Amend LS) and the “Segment Local Set” (Segment LS) enable data preservation and data sharing respectively. Each construct is designed to signal certain types of operations implemented.

The Amend LS and Segment LS constructs are somewhat different from past uses of Local Sets. Within the SMPTE ST 336 document defining KLV, coding of a KLV triplet within the data Value of KLV requires a category designator called groups, which include sets and packs. Both constructs will – by design – contain KLV triplets in their Value, so they are implemented as Local Sets. However, unlike other MISB Local Sets, the Segment LS and Amend LS have no predefined data elements; that is, they are Locals Sets without predefined tags.

In their use, an instantiating metadata set provides the source of metadata data elements. In effect, the Segment LS and Amend LS inherit metadata elements from their parent set based on application. When added to an instantiating MISB metadata set, these KLV constructs provide new functionality, such as metadata editing, metadata preservation and metadata sharing, thereby extending the instantiating metadata set to meet new application needs.

# 7 Amend Local Set

The Amend Local Set enables edits to be made on first-generation metadata, while preserving first-generation metadata. Using a tree hierarchical model, first-generation metadata lies at the root level; changes to first-generation metadata are maintained in one or more child (branch) levels as shown in Figure 1. Multiple parent/child relations can be constructed using this model.

Metadata Set = [Parent] OR [Parent + Child 1] OR [Parent + Child 2] OR … [Parent + Child N]

Figure 1: Parent/Child levels of metadata using the Amend Local Set.

First-generation metadata remains unaltered at the root level, and thus preserved. Within a child, metadata elements defined in the instantiating metadata set can be deleted, added or changed in value (These operations are described in the Motion Imagery Handbook). The user interprets the metadata in its first-generation metadata state, or a combination of first-generation metadata plus any edits of metadata within one of the child sets.

As an example, assume two processes operate on the first-generation metadata resulting in metadata element values considered more accurate. The first-generation metadata needs to be preserved, so those values cannot be changed directly. Using the Amend LS, metadata elements from the parent metadata set can be included into the Amend LS, where those metadata elements can assume their new values. Multiple uses of the Amend LS provide multiple child sets, as Figure 1 illustrates.

The Amend Local Set 16-Byte Universal Label is registered in MISB ST 0807 [2] as:

06.0E.2B.34.02.0B.01.01.0E.01.03.03.03.01.00.00 (CRC 17182)

Figure 2 illustrates a parent set and an Amend LS with updates. Parent metadata Elements, with tag ID’s A, B and C, are the first-generation metadata. Metadata elements with different values for tags A, B and C are denoted as A’, B’ and C’; these are reused and added as child elements in the Amend LS labeled Tag Z. Although the tag identifiers are the same, the elements in the child level have different values than those in the parent level. The resulting metadata exposed to a user is a choice in either Tags A, B, C or Tags A’, B’, C’.

Figure 2: Parent/Child example – Amend LS.

Although this example has only one child, the Amend LS can be reused multiple times within a metadata set for multiple instances of changes to first-generation metadata, thus constructed using multiple child levels. It can also be nested within a child to form a sub-child relationship.

# 8 Segment Local Set

The Segment Local Set defines “branches” of independent metadata, while sharing a common set of metadata from the parent set. A parent set can have one or more Segment Local Sets embedded within it defining parallel sets (branches) of metadata. Each Segment Local Set signals a “union and override” of metadata elements with the list of parent metadata elements. The “union and override” means the list of elements in the Segment Local Set combine with the list of elements from the parent set forming a new set for users to interpret. However, when there are duplicates, the elements in the Segment Local Set take precedence in the resulting set, i.e. they override the elements from the parent set as applied to their localized use within that child set.

With one or more Segment Local Sets, a parent’s metadata elements are reused, or shared, with each child Segment Local Set. Figure 3 illustrates an example parent set with three Segment Local Sets. The parent set provides common elements that each Segment Local set will union and override. The bottom of the figure shows how the user needs to interpret the result, and in this case, there are three independent resulting metadata sets.

Figure 3: Parent/Children level branching using the Segment Local Set.

As an example, assume two images from two sensors on the same platform are formatted into one composite image. Each share common metadata regarding the platform (i.e. elevation, position, etc.), while each have metadata unique to each sensor (i.e. MIIS, sensor type, etc.). The unique metadata elements override the same element identifiers defined in the parent set but with different values, along with other elements available in the parent set if needed.

The Segment LS 16-Byte Universal Label is registered in MISB ST 0807 [2] as:

06.0E.2B.34.02.0B.01.01.0E.01.03.03.03.00.00.00 (CRC 29742)

To illustrate the use of the Segment LS, in Figure 4 metadata elements with Tag ID’s A, B and C are defined at the parent level; these are common across the parent Local Set.

Figure 4: Parent/Child example – Segment LS.

Within the Segment LS, metadata elements with Tag ID’s A’ and C’ are reused; the metadata element with Tag ID F is added from the parent Local Set. All elements at the child level have values which are independent of those at the parent level. The resulting metadata set is a “union and replace” of the common metadata elements specified at the parent level together with the unique metadata elements specified at the child level.

# 9 Limitations

There are certain use cases where using the Amend LS and Segment LS, particularly the Amend LS, may present issues; two are identified below.

- Duplicated metadata elements (i.e. Tags) in a parent set: When multiple instances of the same metadata element appear in a parent metadata set, there is no defined mechanism to discern which metadata elements within an Amend LS or Segment LS relate to a specific instance in the parent set. This is likely not an issue when ST 0601 is the parent set, since duplicate elements are not likely; however, it is anticipated these constructs may be added to other MISB metadata sets in the future, where duplicate elements in a parent set are likely.

- Ordered metadata elements in a parent set: In metadata sets where KLV pack structures are defined, certain packs mandate a specific ordering of their elements. Should such a pack be reused in a child set, the element ordering must be preserved.

Therefore, caution needs to be exercised in employing these constructs in consideration of their intended scope and resulting interpretation. Future guidance will be added to this document at such time the Amend LS and Segment LS constructs are deployed in metadata sets where these issues may arise.

# 10 Rules Governing the Amend LS and Segment LS

A security set (i.e. MISB ST 0102 [4]) is allowed only in the root level of a parent/child set of elements. Two metadata elements within the ST 0102, namely Object Country Coding Method (Tag 12) and Object Country Codes (Tag 13) may be populated within a child level depending on the imaged collection space. When all data within an image references a common country, the Object Country Coding Method and Object Country Codes can be represented at the parent level.

| Requirement(s) |  |
|---|---|
| ST 1607-01 | Where an Amend Local Set is included in a KLV metadata set, the security metadata for the root KLV metadata set shall apply to all of the metadata elements in the metadata set, children and sub-children. |
| ST 1607-02 | Where a Segment Local Set is included in a KLV metadata set, the security metadata for the root KLV metadata set shall apply to all metadata elements in the metadata set, children and sub-children. |
| ST 1607-03 | Where a Segment Local Set is included in a KLV metadata set, the MISB ST 0102 security metadata Local Set elements Tag 1, Tag 2, Tag 3, Tag 4, Tag 5, Tag 6 and Tag 22 (or equivalent), shall be present in the root level of the KLV metadata set. |
| ST 1607-04 | Where metadata within a Segment Local Set indicates a security set object country code unique to that metadata, the MISB ST 0102 security metadata Local Set elements Tag 12 and Tag 13 (or equivalent) shall be present in the metadata described by that Segment Local Set. |

Motion Imagery conformance as set forth by the MISP [5] must be met for all Motion Imagery streams created using distributed metadata elements resulting from building hierarchical parent/child metadata constructs.

| Requirement(s) |  |
|---|---|
| ST 1607-05 | Where an Amend Local Set is included in a KLV metadata set, the root metadata set shall have all metadata elements required to meet MISP conformance. |
| ST 1607-06 | Where a Segment Local Set is included in a KLV metadata set, the union of root, child and sub-child metadata sets metadata elements shall meet MISP conformance. |
