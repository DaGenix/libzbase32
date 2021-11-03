pub(crate) struct TestCase {
    pub unencoded: &'static [u8],
    pub encoded: &'static str,
    pub bits: u64,
}

// Test vectors from: https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt
pub(crate) const STANDARD_TEST_DATA: &'static [TestCase] = &[
    TestCase {
        unencoded: &[0x00],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[0x80],
        encoded: &"o",
        bits: 1,
    },
    TestCase {
        unencoded: &[0x40],
        encoded: &"e",
        bits: 2,
    },
    TestCase {
        unencoded: &[0xc0],
        encoded: &"a",
        bits: 2,
    },
    TestCase {
        unencoded: &[0x00, 0x00],
        encoded: &"yy",
        bits: 10,
    },
    TestCase {
        unencoded: &[0x80, 0x80],
        encoded: &"on",
        bits: 10,
    },
    TestCase {
        unencoded: &[0x8b, 0x88, 0x80],
        encoded: &"tqre",
        bits: 20,
    },
    TestCase {
        unencoded: &[0xf0, 0xbf, 0xc7],
        encoded: &"6n9hq",
        bits: 24,
    },
    TestCase {
        unencoded: &[0xd4, 0x7a, 0x04],
        encoded: &"4t7ye",
        bits: 24,
    },
    TestCase {
        unencoded: &[0xf5, 0x57, 0xbb, 0x0c],
        encoded: &"6im5sd",
        bits: 30,
    },
];

/*
Random test data generated with this Python code:

   from hashlib import sha256
   import zbase32
   import struct


   def rand_int(val):
       h = sha256(struct.pack(">L", val))
       return struct.unpack(">L", h.digest()[0:4])[0]


   def gen(val):
       length = rand_int(val) % 16 + 1
       data = [rand_int(val + 1 + x) % 256 for x in range(length)]
       data_bytes = bytes(bytearray(data))
       bits = (len(data_bytes) - 1) * 8 + (rand_int(val + 50) % 8 + 1)
       result = zbase32.b2a_l(data_bytes, bits)
       data = [ord(x) for x in zbase32.a2b_l(result, bits)]
       return """    TestCase {{
           unencoded: &{data},
           encoded: &"{result}",
           bits: {bits},
       }},""".format(data=data, bits=bits, result=result)


   def main():
       for x in range(1000):
           print(gen(x * 1000))


   if __name__ == "__main__":
       main()
*/
pub(crate) const RANDOM_TEST_DATA: &'static [TestCase] = &[
    TestCase {
        unencoded: &[168, 91, 18, 227, 242, 143, 224, 3, 195],
        encoded: &"ibptfa91t9oy8oa",
        bits: 72,
    },
    TestCase {
        unencoded: &[218, 163, 1, 150, 242, 179, 241, 26, 64],
        encoded: &"5ktodfz1sxatwo",
        bits: 69,
    },
    TestCase {
        unencoded: &[
            195, 39, 117, 153, 45, 253, 111, 172, 219, 159, 68, 192, 192, 186, 248,
        ],
        encoded: &"acuzmgjp9iz43sh9euycbqza",
        bits: 117,
    },
    TestCase {
        unencoded: &[251, 17, 139, 168, 122, 92, 208, 217, 5, 208, 239, 60],
        encoded: &"9ceazkd4muep1bqo7h6",
        bits: 94,
    },
    TestCase {
        unencoded: &[
            107, 41, 125, 71, 146, 117, 186, 186, 102, 144, 131, 161, 172, 232, 41, 223,
        ],
        encoded: &"pcwz4th1qs7mw3wooqo434bj5h",
        bits: 128,
    },
    TestCase {
        unencoded: &[230, 40, 158, 18, 166, 247, 12, 231, 96, 254, 199, 139, 64],
        encoded: &"hawjhrig6hgqqa86a6fw",
        bits: 99,
    },
    TestCase {
        unencoded: &[55, 20, 121, 209, 86, 67, 27, 152],
        encoded: &"ghk8uwksecp3o",
        bits: 61,
    },
    TestCase {
        unencoded: &[156, 26, 59, 155, 166, 0, 23, 41, 50, 11, 214, 156, 224],
        encoded: &"uopdzg7gyym11com44qq",
        bits: 100,
    },
    TestCase {
        unencoded: &[242, 109, 56],
        encoded: &"6jsuo",
        bits: 21,
    },
    TestCase {
        unencoded: &[33, 208, 0, 100, 2, 102, 154, 26, 97, 122, 208],
        encoded: &"r8eyy3ync4pbwam44y",
        bits: 86,
    },
    TestCase {
        unencoded: &[48, 202, 61, 204, 90, 255, 54, 32],
        encoded: &"gdfd5un49h5n",
        bits: 60,
    },
    TestCase {
        unencoded: &[40],
        encoded: &"fy",
        bits: 8,
    },
    TestCase {
        unencoded: &[244, 128, 178, 221, 161, 0, 144],
        encoded: &"61ymfzpbyne",
        bits: 52,
    },
    TestCase {
        unencoded: &[6, 247, 3, 211, 156, 227, 128],
        encoded: &"y55o8whhhq",
        bits: 50,
    },
    TestCase {
        unencoded: &[229, 113, 29, 81, 74, 190],
        encoded: &"hiat4wkkza",
        bits: 47,
    },
    TestCase {
        unencoded: &[
            150, 232, 196, 71, 181, 165, 168, 182, 51, 10, 28, 42, 67, 132,
        ],
        encoded: &"15wcet7iwswmccakdoir8b",
        bits: 110,
    },
    TestCase {
        unencoded: &[151, 129, 128, 194, 160],
        encoded: &"16yaboiy",
        bits: 37,
    },
    TestCase {
        unencoded: &[211, 76, 183, 131, 234, 12, 53, 160, 100, 118, 45, 128],
        encoded: &"4pgmxy9kbo44y3dsfs",
        bits: 89,
    },
    TestCase {
        unencoded: &[147, 180],
        encoded: &"1q4",
        bits: 14,
    },
    TestCase {
        unencoded: &[4, 64],
        encoded: &"yty",
        bits: 11,
    },
    TestCase {
        unencoded: &[77, 134, 50, 228, 137, 45, 203, 24, 74, 207, 210, 9, 110, 0],
        encoded: &"jsddf3rjfzfto1sx4ershy",
        bits: 106,
    },
    TestCase {
        unencoded: &[77, 167, 143, 0],
        encoded: &"jsua6y",
        bits: 26,
    },
    TestCase {
        unencoded: &[169, 142, 13, 76, 78, 12, 128],
        encoded: &"ig8y4unqb1",
        bits: 50,
    },
    TestCase {
        unencoded: &[84, 208, 103, 180, 110, 80, 43, 74, 225, 2],
        encoded: &"kuegxpdqkyiwiaen",
        bits: 79,
    },
    TestCase {
        unencoded: &[192],
        encoded: &"a",
        bits: 4,
    },
    TestCase {
        unencoded: &[253, 25, 109, 207, 43, 38, 43, 203, 207],
        encoded: &"9wcs5u3mraihzua",
        bits: 72,
    },
    TestCase {
        unencoded: &[
            60, 26, 63, 106, 175, 211, 65, 180, 9, 58, 102, 69, 103, 159, 255, 64,
        ],
        encoded: &"8opd64ix4py5enj4c3nsx899e",
        bits: 123,
    },
    TestCase {
        unencoded: &[39, 37, 128],
        encoded: &"rh1a",
        bits: 20,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 1,
    },
    TestCase {
        unencoded: &[214, 2, 145, 87, 247, 240],
        encoded: &"4abjni9z6",
        bits: 44,
    },
    TestCase {
        unencoded: &[215, 238, 59, 153, 192],
        encoded: &"49zdzgq",
        bits: 35,
    },
    TestCase {
        unencoded: &[131, 17],
        encoded: &"oceo",
        bits: 16,
    },
    TestCase {
        unencoded: &[168, 123, 192],
        encoded: &"ib7h",
        bits: 19,
    },
    TestCase {
        unencoded: &[151, 107, 191, 167, 118, 140],
        encoded: &"17i59j5sto",
        bits: 48,
    },
    TestCase {
        unencoded: &[95, 240],
        encoded: &"m9a",
        bits: 13,
    },
    TestCase {
        unencoded: &[105, 227, 36, 71, 57, 88, 128],
        encoded: &"p8t1et33mn",
        bits: 49,
    },
    TestCase {
        unencoded: &[96, 180, 68],
        encoded: &"cn4re",
        bits: 22,
    },
    TestCase {
        unencoded: &[43, 98, 222, 231, 48],
        encoded: &"fptp733o",
        bits: 36,
    },
    TestCase {
        unencoded: &[139, 139, 143, 144, 72, 243, 64],
        encoded: &"tqfa9rne6py",
        bits: 53,
    },
    TestCase {
        unencoded: &[192, 243, 166, 71, 126, 22, 80, 135, 103, 38, 84, 96],
        encoded: &"ad34ct56n3eeq33gkto",
        bits: 93,
    },
    TestCase {
        unencoded: &[101, 114, 225, 182, 78, 104, 190, 66, 3, 98, 176, 29, 192],
        encoded: &"ci3qdp1qpn9rry5nsyqhy",
        bits: 101,
    },
    TestCase {
        unencoded: &[222, 39, 168, 36, 250, 132, 114, 20],
        encoded: &"5au4oj84ot3be",
        bits: 62,
    },
    TestCase {
        unencoded: &[64, 214, 110, 128],
        encoded: &"edmg7y",
        bits: 26,
    },
    TestCase {
        unencoded: &[230, 38, 127, 50, 137, 174, 212, 208, 128],
        encoded: &"hau86cwji5kpb",
        bits: 65,
    },
    TestCase {
        unencoded: &[
            151, 12, 30, 106, 41, 193, 114, 40, 52, 33, 217, 72, 150, 90, 208,
        ],
        encoded: &"1hgbh4tjaf3nopbb5frjcsso",
        bits: 116,
    },
    TestCase {
        unencoded: &[
            133, 238, 232, 215, 171, 42, 179, 231, 234, 150, 252, 210, 86, 16,
        ],
        encoded: &"ozzqti7mfk36x4ws9ujfcr",
        bits: 108,
    },
    TestCase {
        unencoded: &[58, 43, 220, 113, 254, 98, 128],
        encoded: &"8ei7ahx6ck",
        bits: 49,
    },
    TestCase {
        unencoded: &[120, 160, 0],
        encoded: &"xnoy",
        bits: 18,
    },
    TestCase {
        unencoded: &[57, 116, 88, 36, 195, 113, 198, 195, 138, 199, 139, 162],
        encoded: &"8f4fojgdq8dc8ns8tqt",
        bits: 95,
    },
    TestCase {
        unencoded: &[245, 26, 110, 200],
        encoded: &"6wpg71",
        bits: 29,
    },
    TestCase {
        unencoded: &[149, 45, 239, 128],
        encoded: &"1ws69",
        bits: 25,
    },
    TestCase {
        unencoded: &[20, 205, 239, 88, 26, 19, 133, 56, 85, 5, 238, 32],
        encoded: &"nug66sy4nqnuoief7ao",
        bits: 94,
    },
    TestCase {
        unencoded: &[31, 27, 192, 142, 142, 160, 231, 97, 25, 98, 236],
        encoded: &"dhphbdwqwdusngmn7o",
        bits: 87,
    },
    TestCase {
        unencoded: &[255, 230, 178, 19],
        encoded: &"99umrra",
        bits: 32,
    },
    TestCase {
        unencoded: &[57, 152, 145, 17, 96],
        encoded: &"8gcjnrm",
        bits: 35,
    },
    TestCase {
        unencoded: &[
            209, 185, 156, 250, 244, 164, 92, 61, 191, 13, 201, 64, 185, 225, 51, 24,
        ],
        encoded: &"4gh336zwwtqd5xap3fymuajud",
        bits: 125,
    },
    TestCase {
        unencoded: &[19, 68, 244, 19, 224],
        encoded: &"npnxer9y",
        bits: 38,
    },
    TestCase {
        unencoded: &[
            41, 19, 114, 243, 218, 212, 252, 53, 134, 192, 216, 168, 33, 214, 0,
        ],
        encoded: &"frjzfh644u6dmbsy5nwndioy",
        bits: 117,
    },
    TestCase {
        unencoded: &[10, 94, 196, 157, 126, 82, 160],
        encoded: &"bjxcj8m6kko",
        bits: 53,
    },
    TestCase {
        unencoded: &[43, 112, 251, 43, 3, 42, 37, 247, 44, 246, 128],
        encoded: &"fpaxskadfe19qm8so",
        bits: 84,
    },
    TestCase {
        unencoded: &[211, 150, 36, 192, 52, 253, 178, 176],
        encoded: &"4qmnjobw9s3my",
        bits: 61,
    },
    TestCase {
        unencoded: &[178, 150, 219, 135, 162, 7, 5, 192],
        encoded: &"skmpzb7nyhnh",
        bits: 58,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 3,
    },
    TestCase {
        unencoded: &[19, 165, 197, 173, 218],
        encoded: &"nq1hmmq4",
        bits: 39,
    },
    TestCase {
        unencoded: &[231, 87, 185, 123, 40, 3, 149, 105, 64],
        encoded: &"h7m5163eyqks1o",
        bits: 66,
    },
    TestCase {
        unencoded: &[245, 188, 162, 175, 191, 204, 22],
        encoded: &"6s6kfm793om",
        bits: 55,
    },
    TestCase {
        unencoded: &[160, 164, 242, 16, 6, 115],
        encoded: &"wn1xrrygqc",
        bits: 48,
    },
    TestCase {
        unencoded: &[159, 188, 111, 188, 85, 117, 51, 191, 108],
        encoded: &"u66g9xniqw3565",
        bits: 70,
    },
    TestCase {
        unencoded: &[170, 157, 110, 165, 201, 7, 43, 93, 75, 62, 110],
        encoded: &"ikqs7jqjyhii4136pa",
        bits: 88,
    },
    TestCase {
        unencoded: &[203, 7, 41, 1, 245, 228, 32, 249, 230, 0],
        encoded: &"3cd11yxihooxu3oy",
        bits: 80,
    },
    TestCase {
        unencoded: &[
            210, 219, 68, 166, 216, 96, 84, 157, 192, 43, 15, 79, 186, 219, 112, 160,
        ],
        encoded: &"4mpwjjsacbkj5obmb785is5ow",
        bits: 123,
    },
    TestCase {
        unencoded: &[
            172, 82, 199, 236, 88, 151, 173, 189, 65, 198, 194, 94, 157, 80,
        ],
        encoded: &"itjcx5na16s54oqgajxj4w",
        bits: 109,
    },
    TestCase {
        unencoded: &[210, 88, 68, 38, 104, 166, 78, 43, 28, 32, 50, 91, 184],
        encoded: &"4jcrejuew38ns8bygjp5o",
        bits: 101,
    },
    TestCase {
        unencoded: &[187, 7, 168, 215, 77, 10, 83, 230, 99, 199, 0],
        encoded: &"zcd4ti4pbjj6ca68y",
        bits: 81,
    },
    TestCase {
        unencoded: &[115, 175, 34],
        encoded: &"qqz1r",
        bits: 23,
    },
    TestCase {
        unencoded: &[224],
        encoded: &"h",
        bits: 3,
    },
    TestCase {
        unencoded: &[34, 201, 48, 248, 38, 250, 42, 87, 213, 94, 128],
        encoded: &"rmrub6bg9eifxik6o",
        bits: 81,
    },
    TestCase {
        unencoded: &[255, 92, 50, 134, 181, 71, 28, 131, 40],
        encoded: &"97qdfbiiehqegky",
        bits: 72,
    },
    TestCase {
        unencoded: &[50, 36, 115, 229, 203, 84, 18, 242, 246, 190, 166, 166, 232],
        encoded: &"ge1883qmkojxf7i6w4uqo",
        bits: 103,
    },
    TestCase {
        unencoded: &[232, 64],
        encoded: &"7b",
        bits: 10,
    },
    TestCase {
        unencoded: &[
            48, 115, 246, 161, 178, 194, 101, 35, 244, 65, 196, 209, 122, 191, 64,
        ],
        encoded: &"gb39pep1aj1187nbauezix4",
        bits: 114,
    },
    TestCase {
        unencoded: &[204, 251, 123, 91, 95, 90, 197, 182, 141, 223, 28, 5, 112],
        encoded: &"3u7zss49mmn5pdq9donzy",
        bits: 102,
    },
    TestCase {
        unencoded: &[32, 160],
        encoded: &"rno",
        bits: 11,
    },
    TestCase {
        unencoded: &[192],
        encoded: &"a",
        bits: 2,
    },
    TestCase {
        unencoded: &[104, 208, 128],
        encoded: &"pdee",
        bits: 17,
    },
    TestCase {
        unencoded: &[
            83, 99, 210, 192, 62, 165, 153, 48, 61, 196, 65, 220, 185, 131, 128,
        ],
        encoded: &"kpt7fob6wscuyxqre8qmuyh",
        bits: 113,
    },
    TestCase {
        unencoded: &[138, 117, 69, 47, 219, 136, 147, 210, 163, 203, 137, 230, 28],
        encoded: &"tj4wkm65tnj7fe6mt8uba",
        bits: 104,
    },
    TestCase {
        unencoded: &[82, 64, 204],
        encoded: &"kjyca",
        bits: 23,
    },
    TestCase {
        unencoded: &[
            190, 48, 56, 101, 28, 68, 3, 170, 149, 66, 116, 83, 254, 4, 67, 130,
        ],
        encoded: &"zaado3eheob4ifknqtj9hbndoe",
        bits: 127,
    },
    TestCase {
        unencoded: &[220, 58, 140, 189, 59, 227, 249, 20, 44, 5, 185, 65, 215],
        encoded: &"5o7e3xj5hxhtemyfzfy7q",
        bits: 104,
    },
    TestCase {
        unencoded: &[
            23, 71, 47, 195, 224, 26, 89, 250, 38, 53, 237, 35, 182, 85, 102,
        ],
        encoded: &"n7d19o9ydjc9wjti7wt5cimg",
        bits: 119,
    },
    TestCase {
        unencoded: &[65, 128, 72],
        encoded: &"egyro",
        bits: 21,
    },
    TestCase {
        unencoded: &[238, 203, 33, 239, 178, 207, 133, 160],
        encoded: &"75f1d57136n4",
        bits: 60,
    },
    TestCase {
        unencoded: &[250, 67, 200],
        encoded: &"9jbho",
        bits: 23,
    },
    TestCase {
        unencoded: &[4, 202, 199, 249, 3, 135, 130, 128],
        encoded: &"yufcx6edo6be",
        bits: 58,
    },
    TestCase {
        unencoded: &[
            216, 72, 155, 103, 22, 45, 126, 176, 73, 103, 240, 123, 186, 170, 96,
        ],
        encoded: &"5brjs3asfi9my1m86b75ikuy",
        bits: 116,
    },
    TestCase {
        unencoded: &[220, 119, 180, 6, 217, 236],
        encoded: &"5t55ebs37o",
        bits: 48,
    },
    TestCase {
        unencoded: &[167, 73, 179, 97, 200, 200, 45, 184, 27, 36, 31, 71],
        encoded: &"w7r5gaqe3ys5og3rd7do",
        bits: 96,
    },
    TestCase {
        unencoded: &[42, 8, 241, 51, 183, 225, 55, 157, 149, 243, 84, 205, 0],
        encoded: &"ferxnc7zhr535fxukugo",
        bits: 97,
    },
    TestCase {
        unencoded: &[
            62, 55, 218, 102, 105, 197, 42, 240, 224, 178, 220, 88, 184, 208, 192, 0,
        ],
        encoded: &"8a57w3ujawixbaf15tcmtwgyy",
        bits: 121,
    },
    TestCase {
        unencoded: &[
            116, 184, 154, 169, 211, 213, 71, 63, 177, 96, 226, 27, 7, 64,
        ],
        encoded: &"q1hjikqu4idu9cmyhepoqo",
        bits: 108,
    },
    TestCase {
        unencoded: &[19, 133, 56],
        encoded: &"nqnuo",
        bits: 21,
    },
    TestCase {
        unencoded: &[151, 134, 16, 10, 100, 14, 83, 70, 48, 130, 0],
        encoded: &"16dbynurb3jwccrny",
        bits: 81,
    },
    TestCase {
        unencoded: &[168, 126, 86, 61, 250, 110, 110, 61, 238, 64],
        encoded: &"ib9fcxx4p3zd551y",
        bits: 77,
    },
    TestCase {
        unencoded: &[212, 241, 199, 191, 104, 149, 12, 40, 78, 104],
        encoded: &"4uahxx5e1wgnouue",
        bits: 78,
    },
    TestCase {
        unencoded: &[52],
        encoded: &"go",
        bits: 6,
    },
    TestCase {
        unencoded: &[115, 113, 234, 55, 189, 53, 228, 218, 6, 207, 171, 181, 64],
        encoded: &"qpa6wp77gz1pwbsxiq4w",
        bits: 98,
    },
    TestCase {
        unencoded: &[
            224, 210, 217, 161, 116, 254, 89, 246, 189, 191, 101, 20, 218, 220,
        ],
        encoded: &"hdjpuemw93c9pxp9cwkpiz",
        bits: 110,
    },
    TestCase {
        unencoded: &[
            91, 158, 165, 4, 49, 183, 213, 219, 182, 65, 213, 195, 27, 113, 119, 64,
        ],
        encoded: &"mqxkkbbts9k7zp1b4zbtshmzey",
        bits: 128,
    },
    TestCase {
        unencoded: &[
            197, 25, 109, 116, 200, 49, 241, 195, 138, 99, 179, 81, 55, 3, 160,
        ],
        encoded: &"awcs47geg8ah8nudspeuqy7",
        bits: 115,
    },
    TestCase {
        unencoded: &[181, 45],
        encoded: &"swso",
        bits: 16,
    },
    TestCase {
        unencoded: &[178, 245, 18],
        encoded: &"sm4tr",
        bits: 24,
    },
    TestCase {
        unencoded: &[150, 254, 204, 249, 217, 62, 174],
        encoded: &"159c36q384z",
        bits: 55,
    },
    TestCase {
        unencoded: &[203, 149, 115, 0],
        encoded: &"3qkzg",
        bits: 25,
    },
    TestCase {
        unencoded: &[166, 16, 160, 134, 230, 247, 241, 123, 16],
        encoded: &"waekbbzg69azsr",
        bits: 69,
    },
    TestCase {
        unencoded: &[150, 232, 112, 107, 128],
        encoded: &"15w8y4hy",
        bits: 38,
    },
    TestCase {
        unencoded: &[15, 148, 113, 144, 133, 174, 254, 112, 233, 154, 224, 132, 0],
        encoded: &"b6k8drrfi598b4c4hnny",
        bits: 98,
    },
    TestCase {
        unencoded: &[83, 11, 174, 74, 202, 31, 81, 61, 235, 4, 133, 176],
        encoded: &"kcf4h1skd7eu54arosa",
        bits: 93,
    },
    TestCase {
        unencoded: &[52, 99, 164, 152, 105, 163, 0],
        encoded: &"gtt4jgdjwc",
        bits: 49,
    },
    TestCase {
        unencoded: &[202, 92, 241, 171, 67, 50, 229, 152, 0],
        encoded: &"3jqxdk4dgm13oy",
        bits: 66,
    },
    TestCase {
        unencoded: &[36, 9, 44, 252, 196, 57, 10, 125, 23, 230, 169, 4],
        encoded: &"ror139gr8rf84f9girn",
        bits: 95,
    },
    TestCase {
        unencoded: &[252, 77, 171, 89, 176],
        encoded: &"9tg4sspo",
        bits: 36,
    },
    TestCase {
        unencoded: &[
            146, 18, 196, 252, 246, 99, 43, 101, 250, 148, 148, 58, 25, 48, 54,
        ],
        encoded: &"1ejcj98sccism6ww1o7b1cbs",
        bits: 120,
    },
    TestCase {
        unencoded: &[124, 203, 168, 113, 218, 167, 144, 179, 12, 223, 192, 128],
        encoded: &"xuf4ohq4w6emgdg9an",
        bits: 89,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 2,
    },
    TestCase {
        unencoded: &[181, 35, 118, 176, 60, 73, 204, 228],
        encoded: &"swtzpcbhj8gqe",
        bits: 62,
    },
    TestCase {
        unencoded: &[183, 66, 48, 64, 210, 50, 33, 106, 238, 138, 128],
        encoded: &"s7bdyog1geosi5wko",
        bits: 84,
    },
    TestCase {
        unencoded: &[26, 198, 92, 218, 149, 202, 72, 206, 169, 81, 229, 242, 22],
        encoded: &"dmdf3swi3jrc7kkthz3bc",
        bits: 103,
    },
    TestCase {
        unencoded: &[154, 19, 208, 102, 169, 65, 57, 0],
        encoded: &"uej7y3ijerho",
        bits: 57,
    },
    TestCase {
        unencoded: &[
            207, 189, 82, 116, 57, 73, 159, 244, 115, 16, 3, 73, 35, 225, 62,
        ],
        encoded: &"366ir7b3jgx9ehaoypr18aj6",
        bits: 119,
    },
    TestCase {
        unencoded: &[251, 128],
        encoded: &"9qy",
        bits: 15,
    },
    TestCase {
        unencoded: &[125, 108],
        encoded: &"xis",
        bits: 15,
    },
    TestCase {
        unencoded: &[72, 178, 68, 193, 8, 109, 217],
        encoded: &"jn3rjoeepzco",
        bits: 56,
    },
    TestCase {
        unencoded: &[242, 3, 103, 160, 215, 231, 244, 104, 20],
        encoded: &"6ebsxegzh94gof",
        bits: 70,
    },
    TestCase {
        unencoded: &[48, 144, 178, 208, 14, 128],
        encoded: &"gnemfwyqo",
        bits: 41,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"oy",
        bits: 6,
    },
    TestCase {
        unencoded: &[0, 111, 162, 251, 153, 153, 252, 152, 68, 210, 126, 239],
        encoded: &"ybz4f6h3u86jotg1x5zo",
        bits: 96,
    },
    TestCase {
        unencoded: &[136, 208, 49, 6, 168],
        encoded: &"tdednbie",
        bits: 37,
    },
    TestCase {
        unencoded: &[
            201, 71, 140, 247, 4, 105, 4, 53, 167, 190, 39, 199, 141, 128,
        ],
        encoded: &"3fda37arprndmj76r9da5",
        bits: 105,
    },
    TestCase {
        unencoded: &[145, 108, 52, 3, 176, 0],
        encoded: &"1fsdey7oy",
        bits: 43,
    },
    TestCase {
        unencoded: &[238, 181, 105, 47, 100, 218, 39, 34, 42, 184, 71, 252, 128],
        encoded: &"744s1m5r5eu1rkiae96e",
        bits: 97,
    },
    TestCase {
        unencoded: &[34, 175, 128],
        encoded: &"rkza",
        bits: 17,
    },
    TestCase {
        unencoded: &[157, 66, 253, 240, 136, 235, 67, 39, 149, 223, 160, 159, 128],
        encoded: &"uibx5hre7pb1xfq9wnxa",
        bits: 97,
    },
    TestCase {
        unencoded: &[50, 186, 107, 33, 92, 89, 192],
        encoded: &"gk7gsekhm8",
        bits: 50,
    },
    TestCase {
        unencoded: &[
            35, 162, 176, 16, 124, 251, 252, 185, 122, 37, 15, 173, 208, 80, 168,
        ],
        encoded: &"rqtmyrdh9x6m16tfb6s7ywfe",
        bits: 117,
    },
    TestCase {
        unencoded: &[131, 156, 245, 185, 150, 148, 149, 201, 128],
        encoded: &"oqqxmqcs11khu",
        bits: 65,
    },
    TestCase {
        unencoded: &[145, 168, 248],
        encoded: &"1gwxo",
        bits: 21,
    },
    TestCase {
        unencoded: &[
            184, 157, 97, 235, 81, 122, 253, 234, 17, 37, 39, 234, 201, 128,
        ],
        encoded: &"znqsd44txm66wrjfr9icu",
        bits: 105,
    },
    TestCase {
        unencoded: &[24, 39, 164, 174, 208, 90, 68, 242, 10, 45, 156],
        encoded: &"dyu4jmsomjnxrntpuo",
        bits: 87,
    },
    TestCase {
        unencoded: &[191, 27, 119, 43, 208],
        encoded: &"zhpzqk6o",
        bits: 36,
    },
    TestCase {
        unencoded: &[96],
        encoded: &"c",
        bits: 3,
    },
    TestCase {
        unencoded: &[160],
        encoded: &"w",
        bits: 3,
    },
    TestCase {
        unencoded: &[152],
        encoded: &"uy",
        bits: 8,
    },
    TestCase {
        unencoded: &[233, 171, 56, 120, 244, 77, 123, 84, 112],
        encoded: &"7giuo68wji7ieh",
        bits: 68,
    },
    TestCase {
        unencoded: &[183, 70, 141, 171, 220],
        encoded: &"s7de5k6h",
        bits: 38,
    },
    TestCase {
        unencoded: &[
            2, 248, 159, 34, 95, 3, 91, 189, 25, 173, 199, 225, 139, 242, 166, 0,
        ],
        encoded: &"ymhj6e19ypp54gppa9oazhigy",
        bits: 122,
    },
    TestCase {
        unencoded: &[
            112, 154, 55, 101, 189, 21, 171, 221, 94, 228, 26, 43, 75, 107, 136,
        ],
        encoded: &"qnpdq3p7nsi74zzrdeiws4he",
        bits: 118,
    },
    TestCase {
        unencoded: &[104, 194, 249, 67, 1, 138, 146, 34, 163, 88],
        encoded: &"pdbx1oabtkjnfe4a",
        bits: 78,
    },
    TestCase {
        unencoded: &[172, 217, 128],
        encoded: &"iuca",
        bits: 19,
    },
    TestCase {
        unencoded: &[21, 8, 161, 61, 210, 65, 201, 236, 222, 128],
        encoded: &"nwrknxq1e8r63zw",
        bits: 74,
    },
    TestCase {
        unencoded: &[94, 105, 116],
        encoded: &"m3wze",
        bits: 22,
    },
    TestCase {
        unencoded: &[199],
        encoded: &"ah",
        bits: 8,
    },
    TestCase {
        unencoded: &[151, 185, 120, 71, 192],
        encoded: &"16hzot6y",
        bits: 38,
    },
    TestCase {
        unencoded: &[85, 224],
        encoded: &"kzo",
        bits: 14,
    },
    TestCase {
        unencoded: &[
            37, 188, 173, 8, 11, 49, 196, 184, 255, 115, 70, 83, 116, 130, 128,
        ],
        encoded: &"rs6k4nymg8nmt95ue3jzjyw",
        bits: 113,
    },
    TestCase {
        unencoded: &[68, 192],
        encoded: &"eu",
        bits: 10,
    },
    TestCase {
        unencoded: &[
            108, 233, 44, 221, 112, 11, 157, 19, 166, 153, 164, 196, 55, 24,
        ],
        encoded: &"puw13zmobqqt8jw3wundqgy",
        bits: 111,
    },
    TestCase {
        unencoded: &[161, 183, 0],
        encoded: &"wg5o",
        bits: 17,
    },
    TestCase {
        unencoded: &[144, 73, 97, 230, 80, 183, 125, 20],
        encoded: &"1brsd31os76te",
        bits: 63,
    },
    TestCase {
        unencoded: &[112, 64, 3, 136, 104],
        encoded: &"qbyy8nde",
        bits: 37,
    },
    TestCase {
        unencoded: &[58, 165, 158, 235, 68, 128],
        encoded: &"8k13744ro",
        bits: 41,
    },
    TestCase {
        unencoded: &[126, 4, 98, 70, 105, 160, 127, 77, 183, 220, 58, 69, 151, 64],
        encoded: &"xangrtujwb9w5p6h8jn3qo",
        bits: 106,
    },
    TestCase {
        unencoded: &[
            210, 164, 186, 10, 26, 181, 68, 105, 224, 101, 158, 222, 242, 69, 160,
        ],
        encoded: &"4k1mwno4singuadfu5xxrtpy",
        bits: 116,
    },
    TestCase {
        unencoded: &[194, 252, 93, 128],
        encoded: &"am6f5",
        bits: 25,
    },
    TestCase {
        unencoded: &[71, 137, 114, 197, 78, 249, 162, 22, 250, 212],
        encoded: &"e6rzftkq9gtbp6sw",
        bits: 78,
    },
    TestCase {
        unencoded: &[165, 105, 53, 190, 82, 168, 113, 28, 201, 152, 4, 224],
        encoded: &"wiwumx11ibat31cayuo",
        bits: 91,
    },
    TestCase {
        unencoded: &[53, 225, 85, 84, 0],
        encoded: &"gzoikiy",
        bits: 34,
    },
    TestCase {
        unencoded: &[
            148, 121, 131, 245, 105, 82, 106, 25, 170, 150, 194, 35, 98, 11,
        ],
        encoded: &"1tha87mjkjibukwsaetsrna",
        bits: 112,
    },
    TestCase {
        unencoded: &[67, 111, 248, 165, 162, 67, 110, 79, 224],
        encoded: &"epz9tjpnepzr9a",
        bits: 68,
    },
    TestCase {
        unencoded: &[6, 153, 188, 164, 95, 162, 19, 78],
        encoded: &"y4c53jn9wejwh",
        bits: 64,
    },
    TestCase {
        unencoded: &[193, 67, 29, 213, 155, 94, 70, 118, 24, 114, 77, 171, 55, 32],
        encoded: &"afbt5ic5m3d8cgd1jsiuqe",
        bits: 108,
    },
    TestCase {
        unencoded: &[69, 250, 189, 66, 184, 137, 19, 9, 0],
        encoded: &"ez7m4oiatrjo1y",
        bits: 68,
    },
    TestCase {
        unencoded: &[
            18, 138, 83, 243, 184, 72, 38, 48, 189, 172, 219, 119, 23, 158, 166,
        ],
        encoded: &"nkff8h7ajyudbxpc5p5tx8ig",
        bits: 120,
    },
    TestCase {
        unencoded: &[17, 228, 221, 54, 197],
        encoded: &"n81p4psf",
        bits: 40,
    },
    TestCase {
        unencoded: &[13, 199, 174, 81, 235, 87, 184, 11, 192, 173, 238, 178, 160],
        encoded: &"bzd4hwxmk6hyzofp743k",
        bits: 99,
    },
    TestCase {
        unencoded: &[132, 241, 199, 96],
        encoded: &"ouahqa",
        bits: 27,
    },
    TestCase {
        unencoded: &[
            158, 144, 218, 59, 184, 173, 204, 56, 19, 152, 116, 210, 157, 201, 32,
        ],
        encoded: &"u4epwq7aizgdorhaqujj51jy",
        bits: 116,
    },
    TestCase {
        unencoded: &[4, 150, 0],
        encoded: &"y1my",
        bits: 20,
    },
    TestCase {
        unencoded: &[38, 132, 48, 20, 32, 118, 252, 117, 130, 92, 128],
        encoded: &"r4ndyfbyq568my1ho",
        bits: 81,
    },
    TestCase {
        unencoded: &[69, 29, 48, 97, 238, 157, 86, 146, 178, 84, 108, 162, 128],
        encoded: &"ewquyaxquimjfc1wp1te",
        bits: 98,
    },
    TestCase {
        unencoded: &[
            12, 229, 107, 12, 138, 147, 54, 213, 28, 109, 38, 63, 199, 48,
        ],
        encoded: &"bu1ssdrk1c5pk8dpra9hqc",
        bits: 108,
    },
    TestCase {
        unencoded: &[
            51, 177, 63, 22, 183, 211, 215, 166, 224, 10, 254, 246, 73, 38,
        ],
        encoded: &"gqau6fiz4xm4payk955r1jo",
        bits: 111,
    },
    TestCase {
        unencoded: &[207, 176, 16, 32, 45, 128],
        encoded: &"36abyebpo",
        bits: 41,
    },
    TestCase {
        unencoded: &[201, 229, 134, 96],
        encoded: &"381aca",
        bits: 27,
    },
    TestCase {
        unencoded: &[47, 103, 200, 115, 233, 134, 192],
        encoded: &"f7uhoh9jo5",
        bits: 50,
    },
    TestCase {
        unencoded: &[134, 135, 246, 109, 42, 8, 151, 24],
        encoded: &"o4d9c5jkbnmto",
        bits: 62,
    },
    TestCase {
        unencoded: &[133],
        encoded: &"ow",
        bits: 8,
    },
    TestCase {
        unencoded: &[196, 68, 140, 207, 181, 30, 142, 109, 18, 97, 128],
        encoded: &"atne3u7id48g4rubo",
        bits: 81,
    },
    TestCase {
        unencoded: &[133, 14, 69, 85, 64, 18, 63, 25, 67, 186],
        encoded: &"ow8rkikyne9t1o74",
        bits: 79,
    },
    TestCase {
        unencoded: &[62, 193, 131, 203, 6, 224, 47, 66, 63, 243, 93, 105, 28, 128],
        encoded: &"85ya81aghyzwrx9umiwt3y",
        bits: 108,
    },
    TestCase {
        unencoded: &[20, 126, 231, 183, 113, 163, 98, 187, 124, 62, 76],
        encoded: &"nt9qxp5twptms9b6jo",
        bits: 86,
    },
    TestCase {
        unencoded: &[100, 153, 114, 193, 128],
        encoded: &"c1czfoc",
        bits: 34,
    },
    TestCase {
        unencoded: &[99, 160, 90, 198, 228, 67, 83, 245, 111, 152, 90],
        encoded: &"cqofitzrepj9k5hame",
        bits: 87,
    },
    TestCase {
        unencoded: &[
            197, 0, 130, 51, 215, 159, 176, 187, 253, 112, 107, 222, 46, 234, 192,
        ],
        encoded: &"awyerc6zu6amz9mopxxn74sy",
        bits: 118,
    },
    TestCase {
        unencoded: &[195, 217, 59, 222, 41, 113, 211, 221, 179, 151, 151, 128],
        encoded: &"axcuzztjq8j75chz16y",
        bits: 92,
    },
    TestCase {
        unencoded: &[160, 233, 45, 140],
        encoded: &"wdw15d",
        bits: 30,
    },
    TestCase {
        unencoded: &[210, 236, 148, 87, 118, 198, 88, 188, 160],
        encoded: &"4msjei5sa3cm3e",
        bits: 67,
    },
    TestCase {
        unencoded: &[50, 204, 222],
        encoded: &"gmgph",
        bits: 24,
    },
    TestCase {
        unencoded: &[185, 52, 232, 198, 160],
        encoded: &"zr4qtti",
        bits: 35,
    },
    TestCase {
        unencoded: &[212, 234, 47, 157, 103, 98],
        encoded: &"4uin98m8ce",
        bits: 47,
    },
    TestCase {
        unencoded: &[163, 50, 40, 36, 71, 143, 174, 64],
        encoded: &"wc3nojn8t6zr",
        bits: 59,
    },
    TestCase {
        unencoded: &[
            64, 228, 131, 199, 167, 155, 134, 156, 108, 106, 87, 109, 57, 30, 4, 64,
        ],
        encoded: &"ed1e8t78uqdja5dkk7su18ore",
        bits: 124,
    },
    TestCase {
        unencoded: &[
            0, 68, 103, 65, 78, 189, 116, 52, 189, 66, 241, 1, 241, 145, 123, 174,
        ],
        encoded: &"ybngqokqzi4djxkn6ry9drm5ia",
        bits: 127,
    },
    TestCase {
        unencoded: &[165, 0],
        encoded: &"ww",
        bits: 10,
    },
    TestCase {
        unencoded: &[58, 80, 79, 14, 224, 3, 79, 174, 222, 99, 203, 121, 176],
        encoded: &"8jer6dzyyp847zud3ph5y",
        bits: 104,
    },
    TestCase {
        unencoded: &[144],
        encoded: &"1",
        bits: 4,
    },
    TestCase {
        unencoded: &[244, 128],
        encoded: &"61y",
        bits: 12,
    },
    TestCase {
        unencoded: &[48, 44, 78, 207, 255, 34, 45, 75, 246, 16],
        encoded: &"gysr7u99reswz7oo",
        bits: 76,
    },
    TestCase {
        unencoded: &[114, 66, 66, 215, 215, 15, 193, 237, 22, 64],
        encoded: &"qjbrfi6zb9y64f1",
        bits: 74,
    },
    TestCase {
        unencoded: &[195, 224, 133, 192],
        encoded: &"axoemo",
        bits: 26,
    },
    TestCase {
        unencoded: &[75, 74, 188, 241, 242, 179, 192],
        encoded: &"jpfm3hx1sx",
        bits: 50,
    },
    TestCase {
        unencoded: &[71, 223, 194, 32, 136],
        encoded: &"e9xhrere",
        bits: 37,
    },
    TestCase {
        unencoded: &[225, 143, 48, 32],
        encoded: &"hg8uye",
        bits: 28,
    },
    TestCase {
        unencoded: &[112],
        encoded: &"qy",
        bits: 6,
    },
    TestCase {
        unencoded: &[
            122, 54, 33, 168, 137, 70, 117, 59, 26, 122, 80, 140, 102, 184,
        ],
        encoded: &"xe5ndkrje34usgu4knggpq",
        bits: 109,
    },
    TestCase {
        unencoded: &[29, 61, 16, 74, 180],
        encoded: &"dw6ty1iw",
        bits: 38,
    },
    TestCase {
        unencoded: &[113, 227, 168, 57, 187, 76, 104, 236, 96],
        encoded: &"q8t4oqp5jtwqaa",
        bits: 70,
    },
    TestCase {
        unencoded: &[14],
        encoded: &"ba",
        bits: 8,
    },
    TestCase {
        unencoded: &[31, 8, 248, 210, 225, 151, 150, 236, 13, 128],
        encoded: &"dhrxtwzb16mqadc",
        bits: 73,
    },
    TestCase {
        unencoded: &[182, 132, 100, 128],
        encoded: &"s4ngjy",
        bits: 28,
    },
    TestCase {
        unencoded: &[66, 201, 36, 19, 182, 195, 146, 213, 238, 157, 4, 153, 108],
        encoded: &"emr1er7saqjpm5w7y1csa",
        bits: 104,
    },
    TestCase {
        unencoded: &[114, 98, 177, 128],
        encoded: &"qjtmd",
        bits: 25,
    },
    TestCase {
        unencoded: &[125, 235, 48, 239, 122, 61, 16, 192],
        encoded: &"xziub5548wec",
        bits: 58,
    },
    TestCase {
        unencoded: &[
            161, 116, 62, 136, 158, 219, 63, 150, 43, 221, 142, 241, 118, 225, 128,
        ],
        encoded: &"wf4d7nr65c93ck67t5azpac",
        bits: 113,
    },
    TestCase {
        unencoded: &[176],
        encoded: &"sy",
        bits: 7,
    },
    TestCase {
        unencoded: &[
            156, 231, 72, 22, 82, 185, 217, 30, 243, 237, 20, 207, 241, 38, 180,
        ],
        encoded: &"uuuwof11z8ct7h9pnu89njiw",
        bits: 118,
    },
    TestCase {
        unencoded: &[
            123, 201, 141, 90, 110, 155, 34, 78, 241, 84, 164, 234, 115, 40,
        ],
        encoded: &"xxra4suquctr7hkwwui8gk",
        bits: 109,
    },
    TestCase {
        unencoded: &[51, 82, 0, 212, 160],
        encoded: &"gpjybify",
        bits: 36,
    },
    TestCase {
        unencoded: &[176, 80],
        encoded: &"sbe",
        bits: 12,
    },
    TestCase {
        unencoded: &[98, 253, 9, 161, 187, 187, 232, 48, 192],
        encoded: &"cm6ouep5zxwdbo",
        bits: 66,
    },
    TestCase {
        unencoded: &[
            130, 250, 162, 183, 190, 254, 153, 32, 222, 72, 119, 170, 132, 160,
        ],
        encoded: &"om7kfp7694c1bz1eq6ieje",
        bits: 107,
    },
    TestCase {
        unencoded: &[219, 148, 111, 199, 232, 28, 213, 212, 128],
        encoded: &"5qkg9t9eduk7jy",
        bits: 67,
    },
    TestCase {
        unencoded: &[170, 126, 38, 160],
        encoded: &"ij9npe",
        bits: 27,
    },
    TestCase {
        unencoded: &[50, 132, 205],
        encoded: &"gknc4",
        bits: 24,
    },
    TestCase {
        unencoded: &[
            196, 90, 163, 198, 117, 29, 53, 202, 108, 137, 135, 1, 36, 240,
        ],
        encoded: &"atpk8tuidw4hw5rjohy1jh",
        bits: 108,
    },
    TestCase {
        unencoded: &[131, 85, 182, 128],
        encoded: &"opk5py",
        bits: 27,
    },
    TestCase {
        unencoded: &[
            37, 33, 33, 49, 209, 10, 95, 247, 188, 199, 155, 136, 161, 192,
        ],
        encoded: &"rwo1ncqtbjx9xxg8uqrkdo",
        bits: 106,
    },
    TestCase {
        unencoded: &[99, 96, 160],
        encoded: &"cpok",
        bits: 20,
    },
    TestCase {
        unencoded: &[60, 0],
        encoded: &"8oy",
        bits: 11,
    },
    TestCase {
        unencoded: &[
            85, 239, 242, 10, 130, 71, 3, 93, 169, 147, 110, 241, 75, 98, 95, 38,
        ],
        encoded: &"kzz9rnwnehbi5kcup5awsa19ra",
        bits: 127,
    },
    TestCase {
        unencoded: &[241, 81, 131, 77, 236, 210, 61, 90, 208, 185, 204],
        encoded: &"6feaguxc4e6iiwf33o",
        bits: 88,
    },
    TestCase {
        unencoded: &[145, 113, 83, 33, 200, 8, 243, 79, 0],
        encoded: &"1faigeqebd3w6y",
        bits: 66,
    },
    TestCase {
        unencoded: &[73, 89, 69, 214, 14, 128],
        encoded: &"jfcwmioqo",
        bits: 41,
    },
    TestCase {
        unencoded: &[
            68, 79, 152, 191, 197, 97, 178, 206, 232, 80, 60, 19, 230, 161, 235, 128,
        ],
        encoded: &"et83tx6fcg3c74no8oj6pexmo",
        bits: 123,
    },
    TestCase {
        unencoded: &[113, 169, 49, 170, 16, 166, 79, 92, 152, 144, 128],
        encoded: &"qgwudkoow38i3groo",
        bits: 81,
    },
    TestCase {
        unencoded: &[121, 103, 245, 3, 0],
        encoded: &"xfu9kya",
        bits: 33,
    },
    TestCase {
        unencoded: &[156, 88, 211, 197, 220, 74, 40, 162, 8, 35, 128],
        encoded: &"utcp8tqhjewkrnbdo",
        bits: 83,
    },
    TestCase {
        unencoded: &[139, 91, 11, 228, 84, 64],
        encoded: &"tppoz3nwe",
        bits: 42,
    },
    TestCase {
        unencoded: &[72, 56],
        encoded: &"jyhy",
        bits: 16,
    },
    TestCase {
        unencoded: &[238, 233, 223, 106, 226, 204, 148, 63, 180, 176],
        encoded: &"75w764zn31kd9pfo",
        bits: 77,
    },
    TestCase {
        unencoded: &[
            39, 251, 75, 63, 21, 86, 208, 35, 90, 159, 235, 10, 13, 169, 64,
        ],
        encoded: &"r97wsxaik5engsw97cfy5kk",
        bits: 114,
    },
    TestCase {
        unencoded: &[102, 8, 61, 7, 72, 176],
        encoded: &"card4b4es",
        bits: 44,
    },
    TestCase {
        unencoded: &[105, 201, 48, 101, 231, 251, 194],
        encoded: &"p8ruy3x89xb",
        bits: 55,
    },
    TestCase {
        unencoded: &[
            117, 237, 222, 174, 182, 144, 202, 210, 178, 223, 197, 141, 252, 184,
        ],
        encoded: &"qzs77mis1dfpfcs9asg93qy",
        bits: 111,
    },
    TestCase {
        unencoded: &[144, 1, 44, 235, 57, 25, 19, 98, 60, 205, 132],
        encoded: &"1yy13433drjsrxgpoo",
        bits: 88,
    },
    TestCase {
        unencoded: &[22, 113, 29, 33, 81, 25, 207, 148, 34, 89, 4],
        encoded: &"n3at4ektd883ee13yo",
        bits: 86,
    },
    TestCase {
        unencoded: &[43, 199, 125, 15, 214, 237, 127, 238, 72, 87, 236, 47, 216],
        encoded: &"fxdz4d6s7i96h1nz7oz7o",
        bits: 103,
    },
    TestCase {
        unencoded: &[
            64, 132, 144, 41, 252, 91, 191, 208, 17, 116, 179, 206, 149, 32,
        ],
        encoded: &"ennjykxhmq97yrmwsx8jke",
        bits: 107,
    },
    TestCase {
        unencoded: &[247, 220, 144],
        encoded: &"69qj",
        bits: 20,
    },
    TestCase {
        unencoded: &[209, 26, 80],
        encoded: &"4rpfy",
        bits: 21,
    },
    TestCase {
        unencoded: &[82, 28, 173],
        encoded: &"keqk4",
        bits: 24,
    },
    TestCase {
        unencoded: &[60, 196, 100, 31, 128],
        encoded: &"8unge8h",
        bits: 33,
    },
    TestCase {
        unencoded: &[228, 193, 181, 131, 139, 213, 210],
        encoded: &"huy5myhm4zj",
        bits: 55,
    },
    TestCase {
        unencoded: &[208],
        encoded: &"4",
        bits: 4,
    },
    TestCase {
        unencoded: &[111, 87, 128],
        encoded: &"p7ma",
        bits: 19,
    },
    TestCase {
        unencoded: &[111, 18, 108, 47, 141, 54, 21, 71, 201, 62, 0],
        encoded: &"phjgamhpgakwx1j6y",
        bits: 82,
    },
    TestCase {
        unencoded: &[57, 192],
        encoded: &"88",
        bits: 10,
    },
    TestCase {
        unencoded: &[34, 62, 49, 160],
        encoded: &"re9dde",
        bits: 28,
    },
    TestCase {
        unencoded: &[224, 117, 57, 250, 215, 245, 212],
        encoded: &"hb4uu6sz6zky",
        bits: 56,
    },
    TestCase {
        unencoded: &[96, 204, 199, 192, 223, 159, 93, 2],
        encoded: &"cdgcxog9u7qor",
        bits: 64,
    },
    TestCase {
        unencoded: &[210, 236, 220, 2, 237, 94, 128],
        encoded: &"4mspayzpm4y",
        bits: 51,
    },
    TestCase {
        unencoded: &[159, 138, 125, 154, 26, 95, 58, 72, 247, 166, 238, 5, 224],
        encoded: &"u6f85go4mh7rt77g7an6y",
        bits: 101,
    },
    TestCase {
        unencoded: &[243, 253, 143, 201, 31, 23, 198, 155, 206, 222, 5, 64],
        encoded: &"6x6a91e9n9djzus6yi",
        bits: 90,
    },
    TestCase {
        unencoded: &[143, 112],
        encoded: &"t7a",
        bits: 13,
    },
    TestCase {
        unencoded: &[41, 150, 130, 227, 11, 76, 112, 246, 185, 62, 188, 64],
        encoded: &"fgmefaamjtaxpqj6zty",
        bits: 93,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[198, 27, 206, 72, 161, 128],
        encoded: &"aaphh1fbo",
        bits: 41,
    },
    TestCase {
        unencoded: &[210, 208, 19, 159, 244, 168, 0],
        encoded: &"4meb889wiyy",
        bits: 51,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[
            0, 33, 53, 77, 242, 188, 88, 230, 149, 58, 35, 46, 105, 20, 64,
        ],
        encoded: &"yyoukux1ztcqpfj4rczg1fn",
        bits: 114,
    },
    TestCase {
        unencoded: &[90, 60, 52],
        encoded: &"me6de",
        bits: 22,
    },
    TestCase {
        unencoded: &[28, 252, 72, 53, 46, 251, 18, 96],
        encoded: &"du6ropjq9cjgy",
        bits: 64,
    },
    TestCase {
        unencoded: &[255, 147, 76, 238, 86, 86, 152],
        encoded: &"96jw351sk4c",
        bits: 54,
    },
    TestCase {
        unencoded: &[64, 181, 4, 164, 226, 80],
        encoded: &"en4ojj8nk",
        bits: 44,
    },
    TestCase {
        unencoded: &[135, 75, 92, 220, 78, 128],
        encoded: &"o7fi3znqo",
        bits: 45,
    },
    TestCase {
        unencoded: &[240],
        encoded: &"6",
        bits: 5,
    },
    TestCase {
        unencoded: &[163, 114, 88, 48],
        encoded: &"wp3foc",
        bits: 28,
    },
    TestCase {
        unencoded: &[202, 251, 39, 40],
        encoded: &"3m71qk",
        bits: 30,
    },
    TestCase {
        unencoded: &[
            19, 184, 53, 224, 234, 245, 50, 10, 213, 80, 190, 11, 78, 232,
        ],
        encoded: &"nqhdma8k6w3yiikozafw74y",
        bits: 111,
    },
    TestCase {
        unencoded: &[232, 104, 44, 106, 255, 39, 128],
        encoded: &"7bwna4z9r6",
        bits: 50,
    },
    TestCase {
        unencoded: &[60, 44, 196, 199, 116, 74, 108],
        encoded: &"8oscjt5wjjs",
        bits: 55,
    },
    TestCase {
        unencoded: &[16, 150, 118, 168],
        encoded: &"nnm8pky",
        bits: 31,
    },
    TestCase {
        unencoded: &[8, 222, 253, 68, 234, 1, 177],
        encoded: &"bdxx4t8kygao",
        bits: 56,
    },
    TestCase {
        unencoded: &[
            155, 65, 186, 193, 7, 1, 252, 254, 237, 179, 24, 115, 70, 216, 76, 240,
        ],
        encoded: &"upy5ioe8y86x75pudb3wpsnc6y",
        bits: 128,
    },
    TestCase {
        unencoded: &[96, 197, 54, 209, 0],
        encoded: &"cdnupwey",
        bits: 36,
    },
    TestCase {
        unencoded: &[215, 151, 47, 112, 164],
        encoded: &"46m16hfr",
        bits: 38,
    },
    TestCase {
        unencoded: &[13, 233, 228, 149, 196],
        encoded: &"bzw6jfqr",
        bits: 38,
    },
    TestCase {
        unencoded: &[177, 136, 158, 169, 27, 128],
        encoded: &"sgrj7ke5o",
        bits: 41,
    },
    TestCase {
        unencoded: &[
            224, 47, 202, 117, 8, 168, 182, 192, 116, 188, 39, 75, 185, 192,
        ],
        encoded: &"hyzhw7eein5cy7fhr7f5uo",
        bits: 107,
    },
    TestCase {
        unencoded: &[75, 88, 10, 54, 148, 151, 164, 136, 91, 248],
        encoded: &"jpcywpww161eos9a",
        bits: 78,
    },
    TestCase {
        unencoded: &[14, 119, 140],
        encoded: &"b35aa",
        bits: 22,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 2,
    },
    TestCase {
        unencoded: &[162, 114, 232, 48, 60, 196, 48, 110, 240],
        encoded: &"wj3qocbhaoag7hy",
        bits: 71,
    },
    TestCase {
        unencoded: &[240],
        encoded: &"6",
        bits: 5,
    },
    TestCase {
        unencoded: &[
            71, 193, 245, 66, 179, 40, 128, 103, 156, 167, 247, 63, 59, 154, 206, 204,
        ],
        encoded: &"e9y9koiufnygx8f86h9uzgsq3o",
        bits: 128,
    },
    TestCase {
        unencoded: &[
            108, 121, 73, 161, 69, 219, 235, 114, 77, 96, 127, 250, 162, 33, 83, 128,
        ],
        encoded: &"pthwuekf5xizrumyx97krekuo",
        bits: 123,
    },
    TestCase {
        unencoded: &[114, 194],
        encoded: &"qmb",
        bits: 15,
    },
    TestCase {
        unencoded: &[138, 82, 67, 110, 122, 222, 101, 165, 0, 128],
        encoded: &"tjjrg5u45314kyr",
        bits: 74,
    },
    TestCase {
        unencoded: &[
            146, 58, 49, 225, 48, 169, 136, 149, 182, 36, 62, 89, 172, 77, 220,
        ],
        encoded: &"1e7ddajoigrjmptr83c4auqh",
        bits: 118,
    },
    TestCase {
        unencoded: &[124, 251, 96],
        encoded: &"xu7s",
        bits: 19,
    },
    TestCase {
        unencoded: &[2, 227, 106, 31, 198, 136, 96, 223, 178, 60, 31, 248, 192],
        encoded: &"ymtsw86gtbop9cthd9hc",
        bits: 98,
    },
    TestCase {
        unencoded: &[184, 82, 133, 240, 128],
        encoded: &"zbjemhr",
        bits: 33,
    },
    TestCase {
        unencoded: &[
            17, 80, 25, 119, 207, 113, 46, 38, 82, 177, 174, 208, 126, 146, 224, 16,
        ],
        encoded: &"nfeb176xqrzncwiti5e87rzyny",
        bits: 126,
    },
    TestCase {
        unencoded: &[14, 3, 128],
        encoded: &"baba",
        bits: 17,
    },
    TestCase {
        unencoded: &[153, 128, 18, 82, 149, 144],
        encoded: &"ugybrwwi1",
        bits: 44,
    },
    TestCase {
        unencoded: &[
            97, 43, 213, 22, 229, 69, 171, 94, 50, 149, 127, 125, 218, 76,
        ],
        encoded: &"cri7kfzfesiihcwix767wuy",
        bits: 111,
    },
    TestCase {
        unencoded: &[145, 25, 5, 250, 63, 182, 222, 171, 128],
        encoded: &"1rcom6t9s5xkzy",
        bits: 68,
    },
    TestCase {
        unencoded: &[123, 8, 20, 180, 183, 120, 234, 194, 86, 117, 224],
        encoded: &"xcrbjpfzxdicriuih",
        bits: 85,
    },
    TestCase {
        unencoded: &[33, 176, 52, 240, 160, 45],
        encoded: &"rgadjhfyfw",
        bits: 48,
    },
    TestCase {
        unencoded: &[
            124, 6, 34, 237, 79, 78, 223, 29, 241, 27, 153, 234, 238, 101, 201, 8,
        ],
        encoded: &"xodnf5kxj5xt5he5u8iqh3qjb",
        bits: 125,
    },
    TestCase {
        unencoded: &[
            191, 162, 198, 129, 196, 69, 118, 50, 254, 41, 99, 70, 115, 181, 0,
        ],
        encoded: &"z6tcpyqrei5df9tjcpd88pe",
        bits: 114,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 1,
    },
    TestCase {
        unencoded: &[21, 128],
        encoded: &"ns",
        bits: 9,
    },
    TestCase {
        unencoded: &[
            133, 219, 10, 143, 189, 119, 210, 34, 223, 128, 237, 25, 64, 235, 149,
        ],
        encoded: &"ozpoid77q9jnfzhy7wcwb4hi",
        bits: 120,
    },
    TestCase {
        unencoded: &[146, 28, 86, 217, 115, 39, 224],
        encoded: &"1eqfpsmur9o",
        bits: 51,
    },
    TestCase {
        unencoded: &[106, 144, 153, 29, 242, 218, 190, 64],
        encoded: &"pkej18x15k9r",
        bits: 58,
    },
    TestCase {
        unencoded: &[16, 221, 69, 182, 90, 164, 68, 60, 27, 75, 0],
        encoded: &"ndqwmp14wtndag4my",
        bits: 81,
    },
    TestCase {
        unencoded: &[4, 215, 124, 177, 40],
        encoded: &"yumz3cje",
        bits: 37,
    },
    TestCase {
        unencoded: &[192],
        encoded: &"a",
        bits: 2,
    },
    TestCase {
        unencoded: &[182, 13, 48, 182, 177, 113, 235, 249, 177, 45, 192],
        encoded: &"sagubpitq8i9ucjpa",
        bits: 83,
    },
    TestCase {
        unencoded: &[10, 111, 252, 230, 131, 24, 11, 254, 66, 21, 128],
        encoded: &"bjz933wddyf9hooio",
        bits: 82,
    },
    TestCase {
        unencoded: &[187, 13, 174, 162, 24, 215, 59, 42, 143, 40],
        encoded: &"zcg47eoa4h71id3e",
        bits: 77,
    },
    TestCase {
        unencoded: &[52, 121, 230, 171, 249, 77, 183, 97, 0],
        encoded: &"gth6pk93js5sny",
        bits: 66,
    },
    TestCase {
        unencoded: &[212, 201, 5, 29, 247, 16],
        encoded: &"4urok8xzny",
        bits: 48,
    },
    TestCase {
        unencoded: &[130, 121, 104, 231, 19, 88, 183, 110, 192],
        encoded: &"ojhst3aumn5s7o",
        bits: 66,
    },
    TestCase {
        unencoded: &[252, 121, 178, 156, 16, 238, 58, 244, 37, 148],
        encoded: &"9th5f8yo7a7xejcw",
        bits: 79,
    },
    TestCase {
        unencoded: &[175, 132, 163, 223, 109, 255, 159, 197, 59, 4, 232, 155, 128],
        encoded: &"i6nk8z5p96xhkqar7npa",
        bits: 97,
    },
    TestCase {
        unencoded: &[246],
        encoded: &"6a",
        bits: 7,
    },
    TestCase {
        unencoded: &[142, 180, 204, 155, 123, 93, 229, 15, 239, 198, 0],
        encoded: &"t44c3g55mz1o956gy",
        bits: 84,
    },
    TestCase {
        unencoded: &[126, 3, 97, 110, 220, 109, 103, 213, 239, 200],
        encoded: &"xabsn5shpiu7m56e",
        bits: 78,
    },
    TestCase {
        unencoded: &[190, 132, 180, 43, 138, 148, 64, 46, 29, 1, 186],
        encoded: &"z4nmekhk1tynh8ebze",
        bits: 87,
    },
    TestCase {
        unencoded: &[120, 147, 6, 130, 170, 91, 175, 103, 214, 34, 173, 137, 144],
        encoded: &"xnjopyikmqzsxitnisr3",
        bits: 100,
    },
    TestCase {
        unencoded: &[128, 140, 186, 69, 185, 92],
        encoded: &"ongmwtp3mo",
        bits: 48,
    },
    TestCase {
        unencoded: &[94, 133, 183, 254, 169, 162, 160],
        encoded: &"m4n5x9ijwko",
        bits: 53,
    },
    TestCase {
        unencoded: &[174, 102, 254, 56],
        encoded: &"i3uxhq",
        bits: 29,
    },
    TestCase {
        unencoded: &[110, 124, 118, 255, 239, 61, 231, 49, 128],
        encoded: &"p368p99x8zuudy",
        bits: 66,
    },
    TestCase {
        unencoded: &[17, 68, 230, 83, 239, 228, 189, 224],
        encoded: &"nfnqcw9xh166",
        bits: 59,
    },
    TestCase {
        unencoded: &[43, 3, 10, 11, 12, 33, 146, 129, 192],
        encoded: &"fcbownacrgjedo",
        bits: 69,
    },
    TestCase {
        unencoded: &[112, 4, 147, 16, 182, 253, 77, 73, 214, 245],
        encoded: &"qynjgrfs9igwuizi",
        bits: 80,
    },
    TestCase {
        unencoded: &[192],
        encoded: &"a",
        bits: 2,
    },
    TestCase {
        unencoded: &[102, 38, 244, 78, 112],
        encoded: &"cauxeuuo",
        bits: 36,
    },
    TestCase {
        unencoded: &[44, 5, 122, 52],
        encoded: &"fonzwp",
        bits: 30,
    },
    TestCase {
        unencoded: &[129, 38, 128],
        encoded: &"orue",
        bits: 17,
    },
    TestCase {
        unencoded: &[196, 132],
        encoded: &"a1n",
        bits: 14,
    },
    TestCase {
        unencoded: &[252],
        encoded: &"9o",
        bits: 8,
    },
    TestCase {
        unencoded: &[6, 163, 199, 19, 210, 239, 195, 201, 214, 208],
        encoded: &"y4thqr6179bhuiso",
        bits: 76,
    },
    TestCase {
        unencoded: &[172, 128, 209, 150, 46, 199, 149, 134, 48],
        encoded: &"i1ypdftqa6kacc",
        bits: 70,
    },
    TestCase {
        unencoded: &[161, 147, 184, 50, 80],
        encoded: &"wgj5oc1o",
        bits: 37,
    },
    TestCase {
        unencoded: &[154, 161, 176, 192],
        encoded: &"uko5bo",
        bits: 28,
    },
    TestCase {
        unencoded: &[137, 220, 193, 182, 26, 140, 88, 64, 107, 109, 200],
        encoded: &"t8qcdpo4ttcry45p3y",
        bits: 87,
    },
    TestCase {
        unencoded: &[
            32, 40, 227, 29, 180, 103, 132, 161, 229, 154, 12, 70, 130, 96,
        ],
        encoded: &"rywqg8pwc6nkd3c4btdera",
        bits: 108,
    },
    TestCase {
        unencoded: &[189, 27, 184, 99, 254, 60, 28],
        encoded: &"zwp5oa968oq",
        bits: 54,
    },
    TestCase {
        unencoded: &[44, 161, 40, 178, 189, 19, 55, 89, 64],
        encoded: &"f1o1tci7nc5i1o",
        bits: 67,
    },
    TestCase {
        unencoded: &[135, 128],
        encoded: &"o6y",
        bits: 12,
    },
    TestCase {
        unencoded: &[29, 60, 134, 0],
        encoded: &"dw6ecy",
        bits: 29,
    },
    TestCase {
        unencoded: &[
            130, 159, 255, 64, 22, 190, 254, 245, 248, 35, 154, 206, 226, 136,
        ],
        encoded: &"okx96oysz59xm6bdum8qfn",
        bits: 109,
    },
    TestCase {
        unencoded: &[75, 113, 126, 51, 219, 192],
        encoded: &"jpazhc65a",
        bits: 44,
    },
    TestCase {
        unencoded: &[115, 225],
        encoded: &"qxoo",
        bits: 16,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 5,
    },
    TestCase {
        unencoded: &[48, 123],
        encoded: &"gb7o",
        bits: 16,
    },
    TestCase {
        unencoded: &[
            220, 45, 200, 116, 60, 128, 164, 122, 232, 67, 179, 26, 93, 17, 0,
        ],
        encoded: &"5osho7bhon18i4ndscpf4re",
        bits: 114,
    },
    TestCase {
        unencoded: &[
            221, 17, 193, 135, 121, 112, 71, 154, 126, 155, 177, 110, 70, 12, 62,
        ],
        encoded: &"5wehdb53qbd3w9w5sfzrcdb6",
        bits: 120,
    },
    TestCase {
        unencoded: &[90, 136, 30, 44, 158, 191, 0, 79, 219, 126, 218],
        encoded: &"mkrbhmr6zhyr9s565e",
        bits: 87,
    },
    TestCase {
        unencoded: &[
            173, 139, 2, 1, 107, 71, 41, 208, 206, 215, 132, 125, 189, 44,
        ],
        encoded: &"isforymmehw7buszot654my",
        bits: 111,
    },
    TestCase {
        unencoded: &[150, 85, 158, 75, 21, 23, 139, 66],
        encoded: &"13k3h1ain6fwr",
        bits: 63,
    },
    TestCase {
        unencoded: &[211, 200, 192],
        encoded: &"4xrc",
        bits: 18,
    },
    TestCase {
        unencoded: &[210, 236, 210, 64],
        encoded: &"4mspro",
        bits: 27,
    },
    TestCase {
        unencoded: &[184, 14, 191, 224],
        encoded: &"zy8m9a",
        bits: 28,
    },
    TestCase {
        unencoded: &[192],
        encoded: &"a",
        bits: 3,
    },
    TestCase {
        unencoded: &[159, 216, 147, 107, 69, 139, 230, 170, 123, 169, 242, 57, 32],
        encoded: &"u9cjg44ftxukw67j6eh1",
        bits: 100,
    },
    TestCase {
        unencoded: &[221, 218, 5, 169, 248, 185, 30, 254, 176],
        encoded: &"5zpymkxazrxx7c",
        bits: 68,
    },
    TestCase {
        unencoded: &[94, 28, 246, 220, 156, 66, 160],
        encoded: &"maqxpzrheko",
        bits: 53,
    },
    TestCase {
        unencoded: &[176],
        encoded: &"s",
        bits: 5,
    },
    TestCase {
        unencoded: &[126, 14, 30, 46, 139, 175, 161, 64, 147, 60, 192],
        encoded: &"xa8bhmwmi6owbr3ha",
        bits: 82,
    },
    TestCase {
        unencoded: &[230, 122, 251, 142, 230, 202, 16, 207, 145, 223, 0],
        encoded: &"h37xzdzg3eec9rq9y",
        bits: 82,
    },
    TestCase {
        unencoded: &[211, 200, 50, 102, 56, 128],
        encoded: &"4xrdr3tao",
        bits: 41,
    },
    TestCase {
        unencoded: &[226, 208, 90, 144, 9, 4, 86],
        encoded: &"hmefiryjytmy",
        bits: 56,
    },
    TestCase {
        unencoded: &[246, 154, 10, 253, 204, 224, 185, 192],
        encoded: &"64pyi9qchnhh",
        bits: 58,
    },
    TestCase {
        unencoded: &[92],
        encoded: &"mo",
        bits: 6,
    },
    TestCase {
        unencoded: &[0, 23, 75, 6, 55, 250, 209, 64],
        encoded: &"yymwsbtz9mew",
        bits: 58,
    },
    TestCase {
        unencoded: &[
            145, 47, 200, 52, 139, 182, 239, 155, 59, 58, 106, 76, 147, 128,
        ],
        encoded: &"1rzhoprms5z3sq34pjgj8y",
        bits: 107,
    },
    TestCase {
        unencoded: &[164, 226, 140, 210, 211, 92, 166, 128],
        encoded: &"wute3wsum1ue",
        bits: 57,
    },
    TestCase {
        unencoded: &[91, 97, 76, 222, 101, 98, 183, 101, 78, 40, 238, 191],
        encoded: &"mpow3zufck5skute749o",
        bits: 96,
    },
    TestCase {
        unencoded: &[164, 109, 160],
        encoded: &"wts4",
        bits: 20,
    },
    TestCase {
        unencoded: &[
            254, 17, 173, 27, 182, 20, 207, 180, 250, 95, 93, 79, 223, 142, 64,
        ],
        encoded: &"9ae44g7snu85j619mi879d1",
        bits: 114,
    },
    TestCase {
        unencoded: &[78, 190, 227, 116, 95, 76],
        encoded: &"j49qg7n9jo",
        bits: 46,
    },
    TestCase {
        unencoded: &[73, 36, 97, 128],
        encoded: &"jr1gd",
        bits: 25,
    },
    TestCase {
        unencoded: &[147, 43, 109, 53, 245, 0],
        encoded: &"1cis4pxiy",
        bits: 42,
    },
    TestCase {
        unencoded: &[100, 145, 32, 244, 117, 80],
        encoded: &"c1e1b7dik",
        bits: 44,
    },
    TestCase {
        unencoded: &[
            226, 96, 248, 11, 168, 116, 19, 187, 112, 167, 43, 51, 126, 176,
        ],
        encoded: &"hjoxon7eqoj5shf8fc3z7c",
        bits: 108,
    },
    TestCase {
        unencoded: &[
            146, 97, 117, 144, 162, 175, 232, 148, 172, 114, 98, 102, 245, 202, 29,
        ],
        encoded: &"1jozmrfni9wjjmd1cjuxm1o7",
        bits: 120,
    },
    TestCase {
        unencoded: &[
            199, 163, 75, 77, 103, 12, 63, 68, 11, 62, 195, 184, 247, 244, 10,
        ],
        encoded: &"a6twsum8bo9wen36aqhxx7yk",
        bits: 119,
    },
    TestCase {
        unencoded: &[239, 7, 207, 246, 81, 141, 109, 146, 184],
        encoded: &"7hdh971ttis3fq",
        bits: 69,
    },
    TestCase {
        unencoded: &[81, 224],
        encoded: &"k8o",
        bits: 11,
    },
    TestCase {
        unencoded: &[248, 187, 18, 70, 33, 166, 85],
        encoded: &"9n7trttbw3ko",
        bits: 56,
    },
    TestCase {
        unencoded: &[200, 167, 0],
        encoded: &"3nuo",
        bits: 17,
    },
    TestCase {
        unencoded: &[119, 226],
        encoded: &"q9t",
        bits: 15,
    },
    TestCase {
        unencoded: &[52, 112, 200],
        encoded: &"gtaco",
        bits: 23,
    },
    TestCase {
        unencoded: &[185, 133, 27, 128],
        encoded: &"zgntzy",
        bits: 26,
    },
    TestCase {
        unencoded: &[245, 133, 189, 67, 115, 154, 196, 64, 0],
        encoded: &"6sn54o5uumnry",
        bits: 65,
    },
    TestCase {
        unencoded: &[197, 128],
        encoded: &"as",
        bits: 9,
    },
    TestCase {
        unencoded: &[176, 184, 230, 203, 83, 224, 70, 125, 243, 22, 128],
        encoded: &"snhqp14uhbd85haso",
        bits: 83,
    },
    TestCase {
        unencoded: &[219, 145, 96],
        encoded: &"5qes",
        bits: 20,
    },
    TestCase {
        unencoded: &[129, 194, 30, 128, 124, 193, 53, 72, 194, 128],
        encoded: &"o8bb7ydhar4wtow",
        bits: 75,
    },
    TestCase {
        unencoded: &[7, 192],
        encoded: &"y9",
        bits: 10,
    },
    TestCase {
        unencoded: &[99, 114, 64, 48, 143, 121, 154, 17, 128],
        encoded: &"cp3rycrxxgpbdy",
        bits: 66,
    },
    TestCase {
        unencoded: &[231, 128, 128],
        encoded: &"h6ye",
        bits: 17,
    },
    TestCase {
        unencoded: &[
            187, 3, 187, 161, 83, 10, 192, 149, 9, 5, 85, 32, 239, 87, 128,
        ],
        encoded: &"zcb5zekubmyjknefkwoq6ih",
        bits: 113,
    },
    TestCase {
        unencoded: &[226, 66, 13, 179, 13, 137, 171, 56, 216, 80, 72],
        encoded: &"hjby5captgiutsnojy",
        bits: 87,
    },
    TestCase {
        unencoded: &[72, 146, 128],
        encoded: &"jnje",
        bits: 19,
    },
    TestCase {
        unencoded: &[
            18, 233, 112, 166, 163, 203, 20, 4, 186, 161, 236, 202, 104, 195, 168, 63,
        ],
        encoded: &"nmwzbjid3ckyjqib7ufgto7e8h",
        bits: 128,
    },
    TestCase {
        unencoded: &[105, 85, 50, 164, 224],
        encoded: &"pfkufj8",
        bits: 35,
    },
    TestCase {
        unencoded: &[187, 120, 6, 125, 231, 89, 128],
        encoded: &"zphyc9x8mgy",
        bits: 53,
    },
    TestCase {
        unencoded: &[47, 180, 208, 176],
        encoded: &"f64pbc",
        bits: 28,
    },
    TestCase {
        unencoded: &[127, 169, 124, 151, 81, 66, 139, 204, 106, 254, 160, 128],
        encoded: &"x6wz3f4tekfha4z6wn",
        bits: 90,
    },
    TestCase {
        unencoded: &[79, 85, 202, 152, 229, 66, 39, 126, 67, 115, 30],
        encoded: &"j7khig8feeuzho5uda",
        bits: 88,
    },
    TestCase {
        unencoded: &[151, 153, 140, 75, 69, 243, 108, 167, 23, 76, 90, 192],
        encoded: &"16caa14f6pskqf4cmmy",
        bits: 94,
    },
    TestCase {
        unencoded: &[
            166, 245, 208, 67, 137, 234, 139, 244, 83, 112, 76, 64, 12, 33, 196, 64,
        ],
        encoded: &"w547yohj7kf9ew5ojtyyaeqre",
        bits: 122,
    },
    TestCase {
        unencoded: &[149, 89, 191, 238, 129, 45, 24, 172, 202, 211, 75, 64],
        encoded: &"1ic595wbfwck31sujpy",
        bits: 92,
    },
    TestCase {
        unencoded: &[159, 193, 160],
        encoded: &"u9y4",
        bits: 20,
    },
    TestCase {
        unencoded: &[
            192, 45, 155, 231, 250, 11, 71, 42, 227, 60, 92, 206, 24, 116, 128,
        ],
        encoded: &"ays3z394bpd1ia3hmu8bo7r",
        bits: 115,
    },
    TestCase {
        unencoded: &[54, 48],
        encoded: &"gaa",
        bits: 12,
    },
    TestCase {
        unencoded: &[85, 72, 192, 195, 170, 125, 245, 89, 83, 51, 95, 164, 128],
        encoded: &"kircbo7kxz4i1w3um61ey",
        bits: 104,
    },
    TestCase {
        unencoded: &[120, 80],
        encoded: &"xbe",
        bits: 14,
    },
    TestCase {
        unencoded: &[143, 222, 76, 212, 128],
        encoded: &"t9xr3ir",
        bits: 33,
    },
    TestCase {
        unencoded: &[119, 110, 237, 155, 202, 38, 24, 128],
        encoded: &"q7zq5g6krace",
        bits: 57,
    },
    TestCase {
        unencoded: &[186, 165, 126, 209],
        encoded: &"zk1z7we",
        bits: 32,
    },
    TestCase {
        unencoded: &[208, 221, 180, 0],
        encoded: &"4dq5e",
        bits: 25,
    },
    TestCase {
        unencoded: &[84, 207, 15, 38, 25, 96, 185, 133, 60, 121, 159, 103, 128],
        encoded: &"ku8o6jo3cnhakxd3u7ua",
        bits: 97,
    },
    TestCase {
        unencoded: &[
            85, 166, 83, 17, 130, 144, 179, 198, 253, 189, 69, 33, 247, 89, 128,
        ],
        encoded: &"ksufgrcn1n3hp9p7ewo9qsc",
        bits: 114,
    },
    TestCase {
        unencoded: &[247, 88, 243, 236, 107, 10, 201, 120, 206, 22, 2, 230],
        encoded: &"67cx85dmbmrztuosymu",
        bits: 95,
    },
    TestCase {
        unencoded: &[25, 212, 136, 36, 198, 98, 64],
        encoded: &"d8keojggcjy",
        bits: 51,
    },
    TestCase {
        unencoded: &[131, 37, 187, 188, 128],
        encoded: &"oc15zxry",
        bits: 36,
    },
    TestCase {
        unencoded: &[148, 212, 165, 138, 152, 133, 62, 255, 248],
        encoded: &"1ukkmnwaow9x96",
        bits: 70,
    },
    TestCase {
        unencoded: &[2, 167, 218, 150, 103, 202, 184],
        encoded: &"yku7ifu83kh",
        bits: 53,
    },
    TestCase {
        unencoded: &[72, 148, 250, 170, 250, 120, 62, 64],
        encoded: &"jnkxikz4xy9r",
        bits: 59,
    },
    TestCase {
        unencoded: &[
            79, 10, 7, 13, 20, 76, 178, 60, 225, 223, 104, 211, 252, 247, 4, 176,
        ],
        encoded: &"jhfyqdewj13d3aq9pdj937arsy",
        bits: 127,
    },
    TestCase {
        unencoded: &[138, 160, 231, 193, 117, 207, 130, 92, 245, 150, 113, 192],
        encoded: &"tkoqxomi36bf37csq8",
        bits: 90,
    },
    TestCase {
        unencoded: &[71, 217, 192, 99, 67, 188, 128],
        encoded: &"e9chya4dz1",
        bits: 49,
    },
    TestCase {
        unencoded: &[162, 248, 48, 231, 15, 198, 33, 0, 204, 118, 183],
        encoded: &"wmhdb3axaaoobudssh",
        bits: 88,
    },
    TestCase {
        unencoded: &[12, 192],
        encoded: &"buy",
        bits: 11,
    },
    TestCase {
        unencoded: &[220, 157, 218, 61, 74],
        encoded: &"51q7wxkk",
        bits: 40,
    },
    TestCase {
        unencoded: &[250, 140, 67, 73, 62, 190, 29, 22, 46, 84, 42],
        encoded: &"9kgrg1j6zaqtcm1wfe",
        bits: 87,
    },
    TestCase {
        unencoded: &[133, 108, 196, 62, 184, 136, 172, 189, 147, 173, 213, 224],
        encoded: &"oiscexiatnsm5r7p4zo",
        bits: 93,
    },
    TestCase {
        unencoded: &[208],
        encoded: &"4",
        bits: 5,
    },
    TestCase {
        unencoded: &[248, 191, 65, 11, 107, 81, 108, 76, 154, 64],
        encoded: &"9n9wnn5mkfsr3g1",
        bits: 74,
    },
    TestCase {
        unencoded: &[64, 0],
        encoded: &"ey",
        bits: 10,
    },
    TestCase {
        unencoded: &[33, 104, 225, 88, 46, 136, 27, 48, 32],
        encoded: &"rfwqnsbqtypuye",
        bits: 68,
    },
    TestCase {
        unencoded: &[95, 18, 48, 141, 127, 128],
        encoded: &"mhjdbdm9o",
        bits: 45,
    },
    TestCase {
        unencoded: &[176, 135, 29, 119, 73, 250, 120, 6, 116, 236, 144],
        encoded: &"sndt474j9jhyc78c1",
        bits: 85,
    },
    TestCase {
        unencoded: &[
            247, 108, 154, 233, 86, 150, 44, 121, 38, 116, 31, 37, 97, 24, 128,
        ],
        encoded: &"67sji4ks1as81juwdh1sngr",
        bits: 113,
    },
    TestCase {
        unencoded: &[112, 10, 20, 193, 74, 6, 200, 253, 64],
        encoded: &"qyfbjokky5rx4o",
        bits: 68,
    },
    TestCase {
        unencoded: &[52, 171, 237, 141, 154, 228, 207, 114, 96],
        encoded: &"g1i65dc4hu8zra",
        bits: 67,
    },
    TestCase {
        unencoded: &[181, 171, 158, 80, 188, 62, 131, 84, 143, 179, 214, 255],
        encoded: &"ssi3hwfh84bijd7u459o",
        bits: 96,
    },
    TestCase {
        unencoded: &[150, 223, 111, 76, 99, 192],
        encoded: &"15xs6udda",
        bits: 42,
    },
    TestCase {
        unencoded: &[
            47, 41, 105, 158, 28, 156, 146, 81, 53, 155, 251, 172, 125, 128,
        ],
        encoded: &"fhwsu8ohu1jfnpc59qs85",
        bits: 105,
    },
    TestCase {
        unencoded: &[62, 20, 246, 186, 229, 205, 192],
        encoded: &"8akxpqzf3zy",
        bits: 52,
    },
    TestCase {
        unencoded: &[176],
        encoded: &"s",
        bits: 5,
    },
    TestCase {
        unencoded: &[209, 148, 179, 194, 171, 134, 0],
        encoded: &"4gkm8oimoa",
        bits: 50,
    },
    TestCase {
        unencoded: &[177, 191, 121, 94, 252, 238, 16],
        encoded: &"sg9z1zzh7ae",
        bits: 53,
    },
    TestCase {
        unencoded: &[193, 203, 41, 189, 128],
        encoded: &"a8f1uxcy",
        bits: 37,
    },
    TestCase {
        unencoded: &[
            103, 181, 132, 18, 191, 148, 134, 71, 240, 9, 206, 53, 181, 139, 0,
        ],
        encoded: &"c64aeri911drxhyj3a45mna",
        bits: 113,
    },
    TestCase {
        unencoded: &[190, 69, 98, 170, 144],
        encoded: &"z3nsfkwo",
        bits: 36,
    },
    TestCase {
        unencoded: &[9, 172, 52, 150, 202, 182, 207, 126, 118, 156, 67, 217, 156],
        encoded: &"bgsdjfsks58zh7whexc3a",
        bits: 104,
    },
    TestCase {
        unencoded: &[154, 232, 24, 88, 195, 14, 231, 52, 234, 151, 13, 229, 144],
        encoded: &"umwbosgdb5uuj4wzbz13",
        bits: 100,
    },
    TestCase {
        unencoded: &[
            78, 113, 110, 131, 115, 38, 46, 72, 107, 34, 81, 81, 46, 18, 98, 120,
        ],
        encoded: &"j3as7y5urazro43nkfe1hrunx",
        bits: 125,
    },
    TestCase {
        unencoded: &[4, 146, 193, 15, 202, 127, 157, 191, 123, 192],
        encoded: &"y1jcnd6kx6q5666",
        bits: 74,
    },
    TestCase {
        unencoded: &[
            185, 103, 167, 215, 223, 154, 216, 207, 176, 218, 231, 92, 96, 64,
        ],
        encoded: &"zfu4xi69umcc9cg4h7qgyo",
        bits: 108,
    },
    TestCase {
        unencoded: &[
            44, 225, 149, 15, 7, 249, 98, 49, 24, 182, 235, 27, 165, 19, 204, 172,
        ],
        encoded: &"fuo3kda89ftdngfs7cp4kr6cio",
        bits: 126,
    },
    TestCase {
        unencoded: &[
            101, 165, 146, 88, 180, 71, 186, 120, 167, 52, 152, 170, 68, 212, 32, 72,
        ],
        encoded: &"cs13rsfwe678tj3wunirjibyj",
        bits: 125,
    },
    TestCase {
        unencoded: &[189, 240, 141, 135, 181, 175, 0],
        encoded: &"zzae5b7iih",
        bits: 49,
    },
    TestCase {
        unencoded: &[158, 13, 212, 26, 248, 9, 129, 95, 81, 221, 192, 223, 6, 112],
        encoded: &"uag7egzabgyi6wq7adxoch",
        bits: 108,
    },
    TestCase {
        unencoded: &[94, 171, 67, 125, 144, 44],
        encoded: &"m4iwg9cofo",
        bits: 47,
    },
    TestCase {
        unencoded: &[
            206, 60, 220, 70, 237, 26, 163, 119, 227, 184, 196, 112, 210, 74, 160,
        ],
        encoded: &"3a6patzpdktzxa7aatapr1i",
        bits: 115,
    },
    TestCase {
        unencoded: &[120, 12, 172, 66, 91, 144],
        encoded: &"xygkao151",
        bits: 44,
    },
    TestCase {
        unencoded: &[201, 137, 239, 223, 78, 174, 137, 239, 41],
        encoded: &"3gr69z4qi4r66ke",
        bits: 72,
    },
    TestCase {
        unencoded: &[127, 190, 106, 0],
        encoded: &"x69gw",
        bits: 25,
    },
    TestCase {
        unencoded: &[145, 228, 93, 221, 160],
        encoded: &"181f5zp",
        bits: 35,
    },
    TestCase {
        unencoded: &[172, 165, 89, 169, 18, 161, 239, 32],
        encoded: &"i11iuke1w8z1",
        bits: 59,
    },
    TestCase {
        unencoded: &[25, 176],
        encoded: &"dga",
        bits: 15,
    },
    TestCase {
        unencoded: &[93, 171, 151, 214, 155, 210, 138, 219, 202, 0],
        encoded: &"msi3xiw54kfpz1o",
        bits: 75,
    },
    TestCase {
        unencoded: &[
            247, 111, 51, 119, 223, 227, 41, 250, 134, 164, 139, 57, 134, 92, 111, 150,
        ],
        encoded: &"67zug769hcw9ibirtchaczdx1a",
        bits: 127,
    },
    TestCase {
        unencoded: &[225, 151, 221, 241, 15, 128],
        encoded: &"hgm75hexo",
        bits: 41,
    },
    TestCase {
        unencoded: &[174, 157, 55, 177, 80, 92, 226, 7, 112],
        encoded: &"i4quxckomutyqh",
        bits: 70,
    },
    TestCase {
        unencoded: &[60, 20, 107, 28, 3, 202, 222, 25, 192],
        encoded: &"8okgs8yd3mxbuo",
        bits: 67,
    },
    TestCase {
        unencoded: &[86, 25, 82],
        encoded: &"kacir",
        bits: 23,
    },
    TestCase {
        unencoded: &[44, 70, 249, 219, 154, 198, 56, 4, 233, 178, 52, 70, 29],
        encoded: &"ftdxush4aahyj4p1gtdb4",
        bits: 104,
    },
    TestCase {
        unencoded: &[246, 5, 99, 54, 197, 62, 29, 0, 68, 14, 131, 127, 61, 112],
        encoded: &"6ansgpsf8aqoytyqop9u4h",
        bits: 109,
    },
    TestCase {
        unencoded: &[138, 41, 114, 21, 24],
        encoded: &"tewzrfea",
        bits: 37,
    },
    TestCase {
        unencoded: &[117, 92, 200, 234, 128, 153, 82, 213, 235, 196, 24, 212],
        encoded: &"qiqct4wyufjpm46rddk",
        bits: 94,
    },
    TestCase {
        unencoded: &[63, 50, 47, 0],
        encoded: &"8h3n6",
        bits: 25,
    },
    TestCase {
        unencoded: &[12, 3, 225, 83, 118, 250, 12, 166, 145, 101, 130],
        encoded: &"bob6nw5s9egkprmfoe",
        bits: 87,
    },
    TestCase {
        unencoded: &[217, 192, 249, 137, 5, 240, 68, 218],
        encoded: &"58yxunef6bnpw",
        bits: 63,
    },
    TestCase {
        unencoded: &[55, 65, 125, 181, 55, 251, 154, 206, 208, 151, 2, 145, 0],
        encoded: &"g7yz5pjz9qpc7wrzykeo",
        bits: 98,
    },
    TestCase {
        unencoded: &[188, 219, 95, 247, 183, 181, 133, 92, 11, 128],
        encoded: &"zupi977zssnianhy",
        bits: 76,
    },
    TestCase {
        unencoded: &[211, 32, 246, 145, 167, 175, 47, 58],
        encoded: &"4coxprp8ihzuw",
        bits: 64,
    },
    TestCase {
        unencoded: &[135, 128],
        encoded: &"o6",
        bits: 10,
    },
    TestCase {
        unencoded: &[152, 84, 203, 27, 171, 145, 64, 40, 128],
        encoded: &"ubkcsg7m1fynty",
        bits: 66,
    },
    TestCase {
        unencoded: &[58, 242, 93, 64, 30, 201, 45, 192],
        encoded: &"8m3f4oy63rsh",
        bits: 59,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 3,
    },
    TestCase {
        unencoded: &[30, 78, 181, 158, 160, 206, 13, 213, 239, 177, 54, 247, 104],
        encoded: &"d38mm8iy3ag7m57tg55so",
        bits: 103,
    },
    TestCase {
        unencoded: &[233, 110, 192, 240, 158, 14, 43, 44, 86, 32],
        encoded: &"7fzcbhr6bai1ait",
        bits: 75,
    },
    TestCase {
        unencoded: &[170, 117, 0],
        encoded: &"ij4o",
        bits: 17,
    },
    TestCase {
        unencoded: &[133, 5, 32, 181, 110, 158, 67, 240],
        encoded: &"own1bpmqu3b9y",
        bits: 61,
    },
    TestCase {
        unencoded: &[220, 218, 112, 210, 235, 189, 176, 199, 128],
        encoded: &"5up8bwzmzsacxy",
        bits: 67,
    },
    TestCase {
        unencoded: &[47, 67, 75, 128],
        encoded: &"f7bwzy",
        bits: 26,
    },
    TestCase {
        unencoded: &[113, 225, 182, 218, 198],
        encoded: &"q8o5pssg",
        bits: 39,
    },
    TestCase {
        unencoded: &[177, 198, 137, 242, 112, 135, 152, 128],
        encoded: &"s8deuhuoo6ce",
        bits: 58,
    },
    TestCase {
        unencoded: &[85, 102, 187, 41, 210, 80],
        encoded: &"kiumskq1k",
        bits: 44,
    },
    TestCase {
        unencoded: &[7, 236, 135, 160],
        encoded: &"y9sexe",
        bits: 27,
    },
    TestCase {
        unencoded: &[
            126, 163, 112, 172, 236, 90, 201, 13, 111, 198, 130, 51, 125, 195, 152,
        ],
        encoded: &"x4tzbm8cmmro456goe3z5oha",
        bits: 117,
    },
    TestCase {
        unencoded: &[
            14, 89, 20, 226, 22, 57, 77, 151, 170, 94, 222, 158, 180, 128,
        ],
        encoded: &"b3ctjaos8fg3xk1654xmj",
        bits: 105,
    },
    TestCase {
        unencoded: &[209, 136, 218, 44, 144],
        encoded: &"4grpwmro",
        bits: 36,
    },
    TestCase {
        unencoded: &[133, 103, 228, 213, 178, 240, 92, 135, 190, 128],
        encoded: &"oiu6jip16bqexxw",
        bits: 73,
    },
    TestCase {
        unencoded: &[45, 106, 200, 224],
        encoded: &"fiicta",
        bits: 27,
    },
    TestCase {
        unencoded: &[89, 166, 81, 92, 197, 108, 136, 0],
        encoded: &"mgufnzgfp1ry",
        bits: 59,
    },
    TestCase {
        unencoded: &[97, 85, 135, 16, 132, 128, 108, 22, 248, 143, 0],
        encoded: &"cfkaqrrrobsbp6rxy",
        bits: 82,
    },
    TestCase {
        unencoded: &[6, 138, 136, 64, 187, 120, 244, 205],
        encoded: &"y4feoof5xd4c4",
        bits: 64,
    },
    TestCase {
        unencoded: &[241, 211, 59, 161, 33, 237, 234, 27, 216],
        encoded: &"68juzejb7zibzs",
        bits: 69,
    },
    TestCase {
        unencoded: &[236, 204, 251, 95, 55, 53, 5, 228, 220, 123, 64],
        encoded: &"7ugxsz3zgwn6jzd5e",
        bits: 83,
    },
    TestCase {
        unencoded: &[224, 64],
        encoded: &"hb",
        bits: 10,
    },
    TestCase {
        unencoded: &[116, 76, 155, 58, 62, 115, 70, 185, 45, 64],
        encoded: &"qtgjsqt6qpdm1mk",
        bits: 74,
    },
    TestCase {
        unencoded: &[192],
        encoded: &"a",
        bits: 4,
    },
    TestCase {
        unencoded: &[
            5, 242, 89, 227, 3, 183, 114, 115, 44, 46, 156, 248, 134, 56, 224, 196,
        ],
        encoded: &"yz3fuaads738gmbquuhecq8yao",
        bits: 126,
    },
    TestCase {
        unencoded: &[112],
        encoded: &"q",
        bits: 4,
    },
    TestCase {
        unencoded: &[
            119, 50, 219, 169, 165, 236, 9, 119, 243, 211, 179, 111, 33, 96,
        ],
        encoded: &"qh3pzkpf7orzxh6uspz1na",
        bits: 107,
    },
    TestCase {
        unencoded: &[26, 62, 192],
        encoded: &"de9c",
        bits: 18,
    },
    TestCase {
        unencoded: &[220, 201, 84, 200, 13, 98, 208],
        encoded: &"5urij1ypcme",
        bits: 53,
    },
    TestCase {
        unencoded: &[92, 15, 241, 22],
        encoded: &"mo89nfo",
        bits: 31,
    },
    TestCase {
        unencoded: &[211, 0, 0, 9, 176],
        encoded: &"4cyyynpo",
        bits: 37,
    },
    TestCase {
        unencoded: &[192],
        encoded: &"a",
        bits: 2,
    },
    TestCase {
        unencoded: &[
            22, 85, 210, 115, 37, 140, 228, 168, 219, 167, 191, 183, 88, 176,
        ],
        encoded: &"n3k7rh3ftu1kts78z65itc",
        bits: 110,
    },
    TestCase {
        unencoded: &[66, 163, 229, 227, 73, 145, 196, 90, 192],
        encoded: &"ekt6ma4j18nfio",
        bits: 68,
    },
    TestCase {
        unencoded: &[
            157, 254, 195, 173, 41, 40, 73, 186, 58, 73, 124, 47, 83, 128,
        ],
        encoded: &"uz9c8mjjfbr5wq1jxozi8",
        bits: 105,
    },
    TestCase {
        unencoded: &[16, 50, 148, 152, 194, 85, 69, 33, 66, 103, 70, 105, 224],
        encoded: &"ny3jjggnkin1nou8e3w6",
        bits: 100,
    },
    TestCase {
        unencoded: &[121, 110, 201, 239, 157, 76, 238, 235, 254, 215, 168, 111, 0],
        encoded: &"xfzcu5h7juzqz9szibzo",
        bits: 97,
    },
    TestCase {
        unencoded: &[
            101, 99, 157, 181, 142, 237, 147, 8, 198, 195, 2, 187, 185, 84,
        ],
        encoded: &"cit35pcq7sjottsdyk751i",
        bits: 110,
    },
    TestCase {
        unencoded: &[23, 0],
        encoded: &"nh",
        bits: 9,
    },
    TestCase {
        unencoded: &[94, 236, 51, 223, 32, 102, 156, 246, 74, 248, 252, 48],
        encoded: &"m5sd8z3yc4qxc1za9oa",
        bits: 92,
    },
    TestCase {
        unencoded: &[121, 19, 140, 250, 109, 121, 121, 238, 164, 101, 176],
        encoded: &"xrja36upxfh67jdfs",
        bits: 84,
    },
    TestCase {
        unencoded: &[241, 20, 61, 28, 164],
        encoded: &"6rkd48fr",
        bits: 40,
    },
    TestCase {
        unencoded: &[164, 2, 244, 64, 173, 96, 233, 165, 224],
        encoded: &"wobxeofpcdw4ma",
        bits: 68,
    },
    TestCase {
        unencoded: &[217, 239, 177, 36, 6, 117, 11, 36, 151, 146, 209, 192, 50, 0],
        encoded: &"58z5njygqwf1jfh148ydry",
        bits: 107,
    },
    TestCase {
        unencoded: &[184, 0],
        encoded: &"zy",
        bits: 10,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[39, 217, 216, 5, 239, 82, 211, 164, 161, 22, 19, 191, 19],
        encoded: &"r9c7obxxkmj4jeesnq9tg",
        bits: 104,
    },
    TestCase {
        unencoded: &[
            142, 198, 254, 33, 109, 86, 214, 94, 42, 158, 105, 99, 135, 13, 192,
        ],
        encoded: &"t5dxhempk5mfhkw6pftaqdq",
        bits: 114,
    },
    TestCase {
        unencoded: &[67, 178, 64],
        encoded: &"eq3r",
        bits: 18,
    },
    TestCase {
        unencoded: &[207, 53, 39, 200, 234, 249, 216, 129, 157, 75],
        encoded: &"3h41x18k98ced8km",
        bits: 80,
    },
    TestCase {
        unencoded: &[
            152, 135, 95, 110, 66, 234, 234, 178, 195, 135, 165, 118, 187, 64, 28,
        ],
        encoded: &"undi651n7mimfoh8wi5msoyh",
        bits: 118,
    },
    TestCase {
        unencoded: &[81, 119, 222, 140],
        encoded: &"kf577dy",
        bits: 31,
    },
    TestCase {
        unencoded: &[248, 192],
        encoded: &"9d",
        bits: 10,
    },
    TestCase {
        unencoded: &[158, 171, 207, 60, 21, 222, 164, 136],
        encoded: &"u4ih6xyi541eo",
        bits: 62,
    },
    TestCase {
        unencoded: &[28, 158],
        encoded: &"d1x",
        bits: 15,
    },
    TestCase {
        unencoded: &[97, 25, 123, 228, 124, 177, 84, 77, 88, 26, 192, 104],
        encoded: &"crczz3dhsfkr4sy4abw",
        bits: 93,
    },
    TestCase {
        unencoded: &[58, 180, 174, 187, 78, 219, 3, 154, 202, 52, 8, 104],
        encoded: &"8k4k7q4q5cb3i1twbbw",
        bits: 93,
    },
    TestCase {
        unencoded: &[74, 104, 150, 74, 193, 163, 227, 79, 53, 175, 100, 21, 216],
        encoded: &"jjwjc1sbwxtw6ppxcok7o",
        bits: 102,
    },
    TestCase {
        unencoded: &[23, 245, 74, 35, 133, 18, 175, 87, 96, 18, 245, 222, 99, 224],
        encoded: &"n94wwehfnkziqay16zxg8a",
        bits: 108,
    },
    TestCase {
        unencoded: &[1, 52, 223, 75, 230, 102, 40, 97, 224, 196, 64],
        encoded: &"yr4p619gcawgdagre",
        bits: 82,
    },
    TestCase {
        unencoded: &[208, 191, 151, 67],
        encoded: &"4n93qoa",
        bits: 32,
    },
    TestCase {
        unencoded: &[68, 68],
        encoded: &"etn",
        bits: 14,
    },
    TestCase {
        unencoded: &[161, 0],
        encoded: &"wry",
        bits: 14,
    },
    TestCase {
        unencoded: &[243, 12, 137, 150, 61, 250, 212, 4, 207, 104],
        encoded: &"6cgeuft79mkyju5e",
        bits: 77,
    },
    TestCase {
        unencoded: &[
            137, 220, 191, 205, 195, 216, 93, 14, 97, 254, 204, 26, 146, 255, 212, 137,
        ],
        encoded: &"t8qm9uqd5bqohax63opjf96wtr",
        bits: 128,
    },
    TestCase {
        unencoded: &[64, 48],
        encoded: &"eya",
        bits: 13,
    },
    TestCase {
        unencoded: &[94, 169, 208, 200, 140, 235, 66, 188, 231, 30, 0],
        encoded: &"m4w7b1rc7pbm33a6y",
        bits: 81,
    },
    TestCase {
        unencoded: &[182, 109, 160, 94, 251, 214, 160],
        encoded: &"s3s4yzz544o",
        bits: 51,
    },
    TestCase {
        unencoded: &[144, 96, 63, 63, 68, 122, 6, 32, 52, 196, 5, 244, 143, 13],
        encoded: &"1bod6x4rxednypgryz4e6de",
        bits: 112,
    },
    TestCase {
        unencoded: &[182, 159, 169, 77, 239, 201],
        encoded: &"s4x41uxx3r",
        bits: 48,
    },
    TestCase {
        unencoded: &[
            59, 203, 6, 76, 38, 76, 78, 127, 23, 213, 217, 3, 71, 127, 96,
        ],
        encoded: &"8xfocubgjt886f6i5rbwq95y",
        bits: 116,
    },
    TestCase {
        unencoded: &[
            76, 233, 173, 215, 58, 48, 2, 70, 54, 214, 158, 102, 51, 56, 140, 192,
        ],
        encoded: &"juw45i34gybrcpssu3udgqrca",
        bits: 123,
    },
    TestCase {
        unencoded: &[195, 3, 208, 37, 62, 247, 231, 221],
        encoded: &"acb7yjj669u74",
        bits: 64,
    },
    TestCase {
        unencoded: &[58, 110, 21, 110, 232, 117, 114, 66, 56],
        encoded: &"8jzbk5zeqi3rrq",
        bits: 69,
    },
    TestCase {
        unencoded: &[91, 112, 8, 124, 87, 79, 110, 83, 48],
        encoded: &"mpayo9nzj7zfgc",
        bits: 68,
    },
    TestCase {
        unencoded: &[3, 152],
        encoded: &"yqc",
        bits: 13,
    },
    TestCase {
        unencoded: &[231, 142, 133, 35, 90, 61, 99, 212, 94, 36, 11, 81, 78, 0],
        encoded: &"h68eke448it7eztrbpewh",
        bits: 105,
    },
    TestCase {
        unencoded: &[184, 245, 28, 203, 108, 171, 205, 237, 128],
        encoded: &"zd4t315cixg65y",
        bits: 67,
    },
    TestCase {
        unencoded: &[176, 166, 225, 22, 131, 81, 162, 217, 51, 31, 17, 74, 192],
        encoded: &"snuqnfwdkgtp1ca9nffc",
        bits: 99,
    },
    TestCase {
        unencoded: &[94, 160, 6, 83, 219, 73, 204, 174, 192, 254, 242, 131, 128],
        encoded: &"m4oycw65j8gk7o866kba",
        bits: 100,
    },
    TestCase {
        unencoded: &[153, 88, 88, 216, 28, 241, 147, 0, 128],
        encoded: &"ufcftsyh6gjoby",
        bits: 66,
    },
    TestCase {
        unencoded: &[151, 65, 64],
        encoded: &"17ywy",
        bits: 23,
    },
    TestCase {
        unencoded: &[198, 73, 0],
        encoded: &"a3ro",
        bits: 17,
    },
    TestCase {
        unencoded: &[22, 220, 210, 131, 250, 66, 63, 64],
        encoded: &"n5qpfy94ee9w",
        bits: 58,
    },
    TestCase {
        unencoded: &[
            245, 63, 85, 124, 11, 88, 226, 10, 172, 47, 162, 119, 15, 28, 68,
        ],
        encoded: &"6w9ik9ymmdtyimbxwj5o68nr",
        bits: 120,
    },
    TestCase {
        unencoded: &[240, 87, 159, 190, 58, 80],
        encoded: &"6bm39xt4k",
        bits: 44,
    },
    TestCase {
        unencoded: &[166, 241, 212, 41, 27, 78, 26, 55, 57, 34, 11, 192, 104, 48],
        encoded: &"w5a7eke5japdqqjnbxygocy",
        bits: 111,
    },
    TestCase {
        unencoded: &[163, 122, 185, 136, 181, 10, 60, 128],
        encoded: &"wp7munfibe6e",
        bits: 57,
    },
    TestCase {
        unencoded: &[26, 162, 242, 24, 193, 161, 195, 126, 160, 139, 153, 221, 20],
        encoded: &"dktxrggbw8bz7ermu8qte",
        bits: 104,
    },
    TestCase {
        unencoded: &[72, 44, 23, 139, 147, 236],
        encoded: &"jysbxnhu7o",
        bits: 46,
    },
    TestCase {
        unencoded: &[140, 96],
        encoded: &"tto",
        bits: 11,
    },
    TestCase {
        unencoded: &[188, 232, 129, 78, 128],
        encoded: &"zuwenuw",
        bits: 35,
    },
    TestCase {
        unencoded: &[170, 97, 107, 31, 228, 29, 172, 21, 108, 176],
        encoded: &"ijoss89rdssbk5fo",
        bits: 76,
    },
    TestCase {
        unencoded: &[210, 168, 213, 141, 46, 1, 122, 8, 143, 168, 154, 128],
        encoded: &"4kwpmdjqyf7ytd7euk",
        bits: 89,
    },
    TestCase {
        unencoded: &[72, 173, 71, 49, 182, 195, 2, 179, 80],
        encoded: &"jnswqcpsacbmgw",
        bits: 68,
    },
    TestCase {
        unencoded: &[169, 87, 176, 1, 227, 179, 245, 93, 232, 69, 128, 152],
        encoded: &"ifm5yyxdsx4i54nfonc",
        bits: 93,
    },
    TestCase {
        unencoded: &[94, 105, 160],
        encoded: &"m3w4y",
        bits: 24,
    },
    TestCase {
        unencoded: &[0, 144],
        encoded: &"yne",
        bits: 12,
    },
    TestCase {
        unencoded: &[225, 219, 248, 240, 239, 23, 225, 148, 75, 132, 8, 202, 64],
        encoded: &"h8p9th8xn9o3e1hrbdfr",
        bits: 99,
    },
    TestCase {
        unencoded: &[204],
        encoded: &"3o",
        bits: 6,
    },
    TestCase {
        unencoded: &[109, 52, 194, 166, 190, 95, 68, 138, 237, 155, 62, 37, 48],
        encoded: &"pw4cfji6m7nei5c58a1uy",
        bits: 101,
    },
    TestCase {
        unencoded: &[246, 124, 253, 235, 195, 16, 199, 111, 116, 162, 16, 116],
        encoded: &"636x546dndds67fnnb4y",
        bits: 96,
    },
    TestCase {
        unencoded: &[
            216, 135, 131, 24, 229, 65, 239, 128, 108, 194, 241, 79, 5, 223, 150, 136,
        ],
        encoded: &"5ndagg8fe8zay5gn6f8omzhsty",
        bits: 126,
    },
    TestCase {
        unencoded: &[117, 234, 254, 72, 217, 37, 170, 222, 161, 0],
        encoded: &"qzixh1g3rsip7ee",
        bits: 74,
    },
    TestCase {
        unencoded: &[
            122, 21, 151, 99, 101, 49, 198, 34, 220, 148, 203, 38, 208, 128,
        ],
        encoded: &"xek3qa5fg8dnfzrw3cupby",
        bits: 106,
    },
    TestCase {
        unencoded: &[28],
        encoded: &"do",
        bits: 6,
    },
    TestCase {
        unencoded: &[50, 154, 42, 57, 189, 202, 136, 67, 246, 168],
        encoded: &"gkpnwqp73krr87ie",
        bits: 79,
    },
    TestCase {
        unencoded: &[45, 192, 206, 99, 13, 255, 219, 72],
        encoded: &"fzychaap99pwo",
        bits: 61,
    },
    TestCase {
        unencoded: &[
            226, 161, 157, 175, 211, 222, 140, 95, 219, 133, 129, 28, 186, 129, 24,
        ],
        encoded: &"hko35m6u54gf9shforqmiyea",
        bits: 117,
    },
    TestCase {
        unencoded: &[0, 62, 241, 252, 62, 137, 31, 192, 65, 128],
        encoded: &"yy9xd9b6trxhyocy",
        bits: 76,
    },
    TestCase {
        unencoded: &[22, 45, 197, 253, 189, 224],
        encoded: &"nashm9p7h",
        bits: 44,
    },
    TestCase {
        unencoded: &[137, 128, 218, 25, 200, 122, 230, 129, 87, 184, 202, 68],
        encoded: &"tgypwgqexmueni7a3jn",
        bits: 95,
    },
    TestCase {
        unencoded: &[196, 92, 153, 222, 135, 86, 228, 24],
        encoded: &"atqjuzw8k51bo",
        bits: 61,
    },
    TestCase {
        unencoded: &[206],
        encoded: &"3a",
        bits: 7,
    },
    TestCase {
        unencoded: &[9, 39, 180, 160, 184, 74, 160, 176, 57, 249, 152, 149, 146],
        encoded: &"bru5jefajkomyqx3unk3r",
        bits: 104,
    },
    TestCase {
        unencoded: &[50, 129, 89, 172],
        encoded: &"gkyiumy",
        bits: 31,
    },
    TestCase {
        unencoded: &[33, 135, 196, 4, 80, 136],
        encoded: &"rgdhebnot",
        bits: 45,
    },
    TestCase {
        unencoded: &[192, 233, 99, 145, 62, 193, 252, 37, 119, 208, 95, 82, 0],
        encoded: &"adws8rj6a86nk76om7jy",
        bits: 97,
    },
    TestCase {
        unencoded: &[188, 128],
        encoded: &"z1",
        bits: 9,
    },
    TestCase {
        unencoded: &[176, 220, 111, 20, 250, 96, 68, 11, 209, 128],
        encoded: &"sdqg6f84cbnyzwc",
        bits: 73,
    },
    TestCase {
        unencoded: &[126, 35, 166, 111, 45, 75, 112, 192, 133],
        encoded: &"xat4c53pjpacbbe",
        bits: 72,
    },
    TestCase {
        unencoded: &[95, 245, 21, 21, 247, 150, 62, 128],
        encoded: &"m94tkfxz1a9e",
        bits: 57,
    },
    TestCase {
        unencoded: &[42, 207, 177, 194, 19, 64],
        encoded: &"fm85dooue",
        bits: 43,
    },
    TestCase {
        unencoded: &[121, 101, 79, 95, 62, 0],
        encoded: &"xf1w6z36y",
        bits: 45,
    },
    TestCase {
        unencoded: &[103, 61, 250, 237, 204],
        encoded: &"ch69i5qc",
        bits: 38,
    },
    TestCase {
        unencoded: &[171, 94, 40, 215, 128],
        encoded: &"ipxntih",
        bits: 34,
    },
    TestCase {
        unencoded: &[223, 152, 200, 192],
        encoded: &"56ccto",
        bits: 29,
    },
    TestCase {
        unencoded: &[247, 203, 112, 8, 215, 152, 238, 238, 196, 204],
        encoded: &"69fzyngzudzq7tgc",
        bits: 78,
    },
    TestCase {
        unencoded: &[46, 180, 236, 211, 84, 208],
        encoded: &"f44q3w4w4",
        bits: 45,
    },
    TestCase {
        unencoded: &[24, 230, 92, 97, 189, 139, 69, 70, 104],
        encoded: &"ddufaap7tpnwc4y",
        bits: 71,
    },
    TestCase {
        unencoded: &[107, 173, 214, 70, 57, 246, 67, 1, 196],
        encoded: &"pqs7ctt363bodty",
        bits: 71,
    },
    TestCase {
        unencoded: &[168, 179, 148, 160],
        encoded: &"in33je",
        bits: 27,
    },
    TestCase {
        unencoded: &[184, 253, 50, 222, 157, 0],
        encoded: &"zd6ufzw7y",
        bits: 42,
    },
    TestCase {
        unencoded: &[152, 241, 37, 159, 58, 80, 12, 250, 164, 139, 199, 224],
        encoded: &"uda1m834kygxijrma9o",
        bits: 91,
    },
    TestCase {
        unencoded: &[217, 218, 226, 168, 64, 152, 10, 242, 123, 109, 214],
        encoded: &"58pqfknyuyfxr65p4a",
        bits: 87,
    },
    TestCase {
        unencoded: &[97, 255, 152],
        encoded: &"c893o",
        bits: 21,
    },
    TestCase {
        unencoded: &[90, 224],
        encoded: &"mmo",
        bits: 13,
    },
    TestCase {
        unencoded: &[233, 226, 34, 157, 150, 182],
        encoded: &"78tnf8cssa",
        bits: 48,
    },
    TestCase {
        unencoded: &[
            166, 250, 20, 191, 217, 103, 206, 239, 187, 106, 159, 91, 25, 224,
        ],
        encoded: &"w57bjx63c98q9q5ku7ptua",
        bits: 109,
    },
    TestCase {
        unencoded: &[
            214, 6, 55, 118, 112, 54, 34, 183, 203, 169, 191, 59, 179, 64, 55, 154,
        ],
        encoded: &"4addq7uogatmx17jzh75gobzue",
        bits: 128,
    },
    TestCase {
        unencoded: &[83, 100, 125, 248],
        encoded: &"kp1856",
        bits: 30,
    },
    TestCase {
        unencoded: &[191, 187, 249, 236, 123, 107],
        encoded: &"z679u5d5pc",
        bits: 48,
    },
    TestCase {
        unencoded: &[156, 86, 133, 232],
        encoded: &"utmem4",
        bits: 29,
    },
    TestCase {
        unencoded: &[95, 82, 104, 88, 27, 2, 80, 135, 179, 72],
        encoded: &"m7jgosy5yjeexc4e",
        bits: 79,
    },
    TestCase {
        unencoded: &[152, 254, 136],
        encoded: &"ud9eo",
        bits: 21,
    },
    TestCase {
        unencoded: &[134, 150, 99, 164, 126, 118, 140, 64],
        encoded: &"o4mg8jd6q4gry",
        bits: 62,
    },
    TestCase {
        unencoded: &[254, 99, 26, 151, 241, 224],
        encoded: &"93ttif9th",
        bits: 44,
    },
    TestCase {
        unencoded: &[95, 87, 0],
        encoded: &"m7mo",
        bits: 17,
    },
    TestCase {
        unencoded: &[154, 52, 138, 138, 223, 12],
        encoded: &"ue4eins9bo",
        bits: 46,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 4,
    },
    TestCase {
        unencoded: &[44, 216, 234, 46, 42],
        encoded: &"fucqwmtk",
        bits: 39,
    },
    TestCase {
        unencoded: &[37, 98, 65, 29, 175, 53, 160, 30, 111, 236, 176, 230, 128],
        encoded: &"ritrn8pxgsobh59csdue",
        bits: 98,
    },
    TestCase {
        unencoded: &[243, 191, 65, 99, 32, 233, 88, 192],
        encoded: &"6q9wna3y7fcc",
        bits: 59,
    },
    TestCase {
        unencoded: &[210, 7, 153, 232, 52],
        encoded: &"4ed3u4bw",
        bits: 39,
    },
    TestCase {
        unencoded: &[
            221, 205, 116, 229, 207, 71, 51, 138, 86, 162, 180, 9, 122, 171, 160,
        ],
        encoded: &"5zgzj3qxeh3awiinsorzik7",
        bits: 115,
    },
    TestCase {
        unencoded: &[92, 18, 208],
        encoded: &"mojp",
        bits: 20,
    },
    TestCase {
        unencoded: &[
            224, 142, 239, 122, 152, 47, 67, 175, 219, 224, 154, 204, 153, 255, 196, 26,
        ],
        encoded: &"hn8q66waf7b49s9yumgju96rde",
        bits: 128,
    },
    TestCase {
        unencoded: &[218, 192, 28, 29],
        encoded: &"5myba8e",
        bits: 32,
    },
    TestCase {
        unencoded: &[
            251, 232, 99, 79, 211, 161, 189, 46, 99, 197, 76, 50, 11, 78, 24, 128,
        ],
        encoded: &"9xwggu6uwg61ha6fjo3ysuoao",
        bits: 121,
    },
    TestCase {
        unencoded: &[120, 224],
        encoded: &"xdo",
        bits: 13,
    },
    TestCase {
        unencoded: &[252, 66, 160],
        encoded: &"9tbky",
        bits: 22,
    },
    TestCase {
        unencoded: &[213, 45, 206, 19, 27, 109, 215, 206, 110, 83, 185, 128],
        encoded: &"4wshhra5pzmhh51uzgy",
        bits: 92,
    },
    TestCase {
        unencoded: &[76, 253, 94, 40, 3, 48, 14, 52],
        encoded: &"ju6ihkydgy8de",
        bits: 64,
    },
    TestCase {
        unencoded: &[243, 22, 38, 110, 76, 249, 24, 82, 129, 151, 48, 81, 186, 82],
        encoded: &"6cmnc51c9rcffyczgbe5wwo",
        bits: 111,
    },
    TestCase {
        unencoded: &[124, 246, 218, 222, 110, 169, 46, 234, 113, 212, 207, 0],
        encoded: &"xu5pizuqirzqwhqw3h",
        bits: 89,
    },
    TestCase {
        unencoded: &[171, 147, 238, 95, 235, 115, 56, 227, 109, 0],
        encoded: &"iqj6hz9mqchqg5e",
        bits: 74,
    },
    TestCase {
        unencoded: &[23, 153, 115, 220, 184],
        encoded: &"n6cz8zfa",
        bits: 37,
    },
    TestCase {
        unencoded: &[156, 58, 162, 233, 236, 80],
        encoded: &"uo7kf4xck",
        bits: 44,
    },
    TestCase {
        unencoded: &[221, 106, 7, 191, 172, 44, 121, 60, 106, 149, 150, 0],
        encoded: &"5iiyxx7cfthua4wi1a",
        bits: 90,
    },
    TestCase {
        unencoded: &[
            93, 56, 7, 22, 238, 32, 162, 162, 21, 166, 248, 5, 213, 106, 48,
        ],
        encoded: &"mwhyqfzqrntkrfpg9yn7k4to",
        bits: 117,
    },
    TestCase {
        unencoded: &[155, 128],
        encoded: &"uq",
        bits: 9,
    },
    TestCase {
        unencoded: &[246, 240, 178, 150, 19, 181, 0],
        encoded: &"65amffousw",
        bits: 50,
    },
    TestCase {
        unencoded: &[242, 5, 204],
        encoded: &"6enha",
        bits: 22,
    },
    TestCase {
        unencoded: &[41, 252, 125, 200, 68, 180, 240],
        encoded: &"f86851nrsuay",
        bits: 56,
    },
    TestCase {
        unencoded: &[78, 207, 15, 199, 20, 133, 209, 255, 177, 104],
        encoded: &"j58o9tawoze99cme",
        bits: 78,
    },
    TestCase {
        unencoded: &[25, 162, 245, 32],
        encoded: &"dgtxke",
        bits: 27,
    },
    TestCase {
        unencoded: &[37, 74, 37, 7, 148, 160],
        encoded: &"rifnkbhww",
        bits: 43,
    },
    TestCase {
        unencoded: &[125, 122, 97, 219, 126, 38, 32, 15, 160],
        encoded: &"xi7gds56raoy9e",
        bits: 67,
    },
    TestCase {
        unencoded: &[191, 52, 62, 84, 82],
        encoded: &"zh4dhin1",
        bits: 40,
    },
    TestCase {
        unencoded: &[33, 186, 227, 96, 51, 90, 27, 12, 207, 122, 124, 248, 192],
        encoded: &"rg7qgabumepo3u54xuhc",
        bits: 98,
    },
    TestCase {
        unencoded: &[151, 84, 185, 115, 107, 50, 51, 99, 13, 137, 64],
        encoded: &"17km1h5mge3sgdcje",
        bits: 83,
    },
    TestCase {
        unencoded: &[150, 107, 49, 116, 142, 191, 27, 172, 170],
        encoded: &"13iun7rqzhp43ko",
        bits: 71,
    },
    TestCase {
        unencoded: &[90, 56, 61, 160],
        encoded: &"mehd5e",
        bits: 27,
    },
    TestCase {
        unencoded: &[64, 243, 12, 180, 15, 156, 220, 240, 113, 32, 36],
        encoded: &"ed3o3pyxuuqxyhjyro",
        bits: 86,
    },
    TestCase {
        unencoded: &[62, 62, 82, 91, 205, 209, 248, 123, 128],
        encoded: &"8a9frs6p48h8z",
        bits: 65,
    },
    TestCase {
        unencoded: &[205, 50, 234, 191, 169, 147, 239, 41, 120],
        encoded: &"3w3qix7j1xz116",
        bits: 69,
    },
    TestCase {
        unencoded: &[31, 90, 42, 41, 57, 40, 241, 16],
        encoded: &"d7pnwkj3fdat",
        bits: 60,
    },
    TestCase {
        unencoded: &[209, 240, 0],
        encoded: &"48ay",
        bits: 18,
    },
    TestCase {
        unencoded: &[114, 73, 94, 114, 126, 197, 37, 79, 128],
        encoded: &"qjrihhu6aw1w9",
        bits: 65,
    },
    TestCase {
        unencoded: &[185, 82, 64],
        encoded: &"zfjr",
        bits: 19,
    },
    TestCase {
        unencoded: &[225, 0],
        encoded: &"hr",
        bits: 9,
    },
    TestCase {
        unencoded: &[234],
        encoded: &"7e",
        bits: 7,
    },
    TestCase {
        unencoded: &[167, 185, 244, 140, 172, 244, 15, 64, 24, 152],
        encoded: &"w6h9jdfc6o8wygra",
        bits: 77,
    },
    TestCase {
        unencoded: &[64],
        encoded: &"ey",
        bits: 7,
    },
    TestCase {
        unencoded: &[204, 128],
        encoded: &"31",
        bits: 9,
    },
    TestCase {
        unencoded: &[11, 185, 80],
        encoded: &"bqhi",
        bits: 20,
    },
    TestCase {
        unencoded: &[
            4, 90, 110, 38, 68, 217, 44, 0, 5, 229, 134, 203, 175, 167, 180, 24,
        ],
        encoded: &"ytpghj1r5rsyybxfo5f49j7wdy",
        bits: 126,
    },
    TestCase {
        unencoded: &[224],
        encoded: &"h",
        bits: 3,
    },
    TestCase {
        unencoded: &[224, 142, 96, 150, 199, 247, 18, 83, 101, 53, 10, 144],
        encoded: &"hn8gbfs86hjfg3jibke",
        bits: 93,
    },
    TestCase {
        unencoded: &[159, 98, 152, 31, 180, 1, 181, 8, 156, 49, 174, 91, 182],
        encoded: &"u7tjo87wyg4ot8bti3p5c",
        bits: 103,
    },
    TestCase {
        unencoded: &[
            88, 52, 160, 31, 94, 157, 158, 217, 247, 34, 200, 211, 200, 160,
        ],
        encoded: &"my4ky846usxpu73n3djhte",
        bits: 107,
    },
    TestCase {
        unencoded: &[
            139, 63, 141, 225, 108, 114, 77, 97, 10, 88, 242, 75, 220, 158, 157,
        ],
        encoded: &"tc9a5amcqjgsnn1a6jf738w7",
        bits: 120,
    },
    TestCase {
        unencoded: &[
            175, 46, 11, 147, 169, 130, 225, 61, 215, 51, 4, 86, 46, 170, 209, 0,
        ],
        encoded: &"ihzyzr7jomou5i3uytmn7ksty",
        bits: 123,
    },
    TestCase {
        unencoded: &[54, 63, 194, 184, 12, 27, 117, 13, 221, 34, 153, 18],
        encoded: &"ga9hfqycdp4o5zjnurjy",
        bits: 96,
    },
    TestCase {
        unencoded: &[
            213, 247, 80, 154, 29, 154, 99, 103, 85, 250, 255, 228, 186, 151, 0,
        ],
        encoded: &"4z5ibgo7ujtsqix4991mifa",
        bits: 113,
    },
    TestCase {
        unencoded: &[211, 166, 229, 205, 41, 150, 32],
        encoded: &"4quqmujj1ao",
        bits: 53,
    },
    TestCase {
        unencoded: &[
            164, 144, 141, 137, 37, 4, 122, 152, 133, 6, 2, 148, 142, 219, 84, 122,
        ],
        encoded: &"w1ee5njfyt7jtbegykke7s4wxe",
        bits: 127,
    },
    TestCase {
        unencoded: &[243, 5, 66, 77, 62, 186, 222, 107, 19, 97, 53],
        encoded: &"6cnwruj6zmxgsr5bgw",
        bits: 88,
    },
    TestCase {
        unencoded: &[60, 246, 35, 195, 81, 79, 206],
        encoded: &"8u5n8o4tj98",
        bits: 55,
    },
    TestCase {
        unencoded: &[123, 10, 0],
        encoded: &"xcfy",
        bits: 20,
    },
    TestCase {
        unencoded: &[36, 55, 145, 204, 136],
        encoded: &"ro53dure",
        bits: 39,
    },
    TestCase {
        unencoded: &[249, 189, 111, 160, 12, 105, 0],
        encoded: &"9g6s9eycpr",
        bits: 49,
    },
    TestCase {
        unencoded: &[194, 215, 33, 136, 138, 9, 146, 83, 168],
        encoded: &"amm1dnrkbgjf8k",
        bits: 70,
    },
    TestCase {
        unencoded: &[130, 147, 33, 237, 116, 87, 3, 9, 148, 0],
        encoded: &"okj1d5mwkhboufy",
        bits: 73,
    },
    TestCase {
        unencoded: &[25, 168, 204, 126, 144, 39, 211, 6, 36, 90, 235, 92, 0],
        encoded: &"dgwca9wor9jocjn47pqy",
        bits: 97,
    },
    TestCase {
        unencoded: &[113, 70, 226, 91, 113, 105],
        encoded: &"qfdqrs5tpr",
        bits: 48,
    },
    TestCase {
        unencoded: &[203, 184, 56, 15, 208, 179, 141, 8],
        encoded: &"3qhdod6osqgoo",
        bits: 61,
    },
    TestCase {
        unencoded: &[20, 56, 81, 191, 49, 41, 93, 9, 219, 220, 177, 80],
        encoded: &"nohfdx3tffqous6hsfe",
        bits: 93,
    },
    TestCase {
        unencoded: &[27, 248],
        encoded: &"dxhy",
        bits: 16,
    },
    TestCase {
        unencoded: &[8, 15, 0],
        encoded: &"by8o",
        bits: 18,
    },
    TestCase {
        unencoded: &[208, 247, 192, 86, 87, 82, 8],
        encoded: &"4d5hyi1zker",
        bits: 54,
    },
    TestCase {
        unencoded: &[157, 253, 2, 83, 90, 200, 170, 134, 167, 22, 48, 192],
        encoded: &"uz6orw443niepjasgdy",
        bits: 94,
    },
    TestCase {
        unencoded: &[
            137, 178, 230, 136, 164, 147, 222, 108, 141, 142, 93, 121, 96,
        ],
        encoded: &"tg3qpnfr1xxg3dcqmihs",
        bits: 99,
    },
    TestCase {
        unencoded: &[24],
        encoded: &"dy",
        bits: 7,
    },
    TestCase {
        unencoded: &[220, 81, 223, 81, 204, 71, 208],
        encoded: &"5te76wqce9e",
        bits: 52,
    },
    TestCase {
        unencoded: &[11, 229, 181, 48],
        encoded: &"bx15kc",
        bits: 29,
    },
    TestCase {
        unencoded: &[241, 232, 13, 112, 4, 130, 131, 3, 55, 192],
        encoded: &"68wy4hyrokbogp6y",
        bits: 76,
    },
    TestCase {
        unencoded: &[131, 149, 176, 177, 112],
        encoded: &"oqk5bcmo",
        bits: 37,
    },
    TestCase {
        unencoded: &[129, 175, 209, 214, 162, 3, 160],
        encoded: &"ogz7diinyqo",
        bits: 51,
    },
    TestCase {
        unencoded: &[53, 24, 115, 97, 4, 54, 0],
        encoded: &"gwc8gaerga",
        bits: 49,
    },
    TestCase {
        unencoded: &[21, 186, 1, 88, 196, 91, 192, 105, 189, 145, 183],
        encoded: &"ns7ynsgrmxyguxctsh",
        bits: 88,
    },
    TestCase {
        unencoded: &[130, 0],
        encoded: &"oe",
        bits: 9,
    },
    TestCase {
        unencoded: &[
            15, 255, 13, 75, 116, 191, 128, 166, 209, 40, 59, 99, 253, 128,
        ],
        encoded: &"b99o415wz6ykpwje8pt95",
        bits: 105,
    },
    TestCase {
        unencoded: &[
            89, 162, 152, 82, 194, 13, 201, 183, 185, 253, 183, 45, 148, 93, 131, 56,
        ],
        encoded: &"mgtjowsnbzr5xqx7shs3ezcd8y",
        bits: 127,
    },
    TestCase {
        unencoded: &[
            26, 77, 91, 28, 64, 171, 59, 98, 17, 122, 66, 64, 218, 187, 88, 32,
        ],
        encoded: &"djgis8nyic7srrm4ejypiq4ar",
        bits: 124,
    },
    TestCase {
        unencoded: &[98, 206, 148, 116, 128],
        encoded: &"cm8je7r",
        bits: 33,
    },
    TestCase {
        unencoded: &[173, 8],
        encoded: &"iwr",
        bits: 13,
    },
    TestCase {
        unencoded: &[78, 62, 169, 163, 64],
        encoded: &"ja9kue4",
        bits: 34,
    },
    TestCase {
        unencoded: &[165, 52, 151, 114, 78, 57, 128],
        encoded: &"ww4jqh1q8gy",
        bits: 53,
    },
    TestCase {
        unencoded: &[236, 134, 131, 228, 96, 12, 23, 47, 147, 37, 8, 236],
        encoded: &"71de83dybom19r3fbds",
        bits: 95,
    },
    TestCase {
        unencoded: &[111, 151, 7, 180, 109, 161, 21, 131, 228, 59, 118, 80],
        encoded: &"p6moxpdpwrka83b5q3e",
        bits: 93,
    },
    TestCase {
        unencoded: &[15, 76, 192],
        encoded: &"b7gc",
        bits: 18,
    },
    TestCase {
        unencoded: &[251, 2, 249, 194, 130, 41, 26, 200, 5, 122, 30, 133, 105, 0],
        encoded: &"9cbxuownfrpcobm4d4ns1y",
        bits: 108,
    },
    TestCase {
        unencoded: &[
            63, 165, 253, 4, 25, 179, 24, 127, 44, 160, 200, 117, 253, 151, 208, 224,
        ],
        encoded: &"86194by3scc86mfy3b495f6oh",
        bits: 123,
    },
    TestCase {
        unencoded: &[61, 4, 201, 235, 44, 81, 182, 36, 33, 66, 183, 126],
        encoded: &"8wncu43ckg5neekns79",
        bits: 95,
    },
    TestCase {
        unencoded: &[96],
        encoded: &"cy",
        bits: 7,
    },
    TestCase {
        unencoded: &[101, 100, 46, 127, 155, 248],
        encoded: &"ci1nh9h59",
        bits: 45,
    },
    TestCase {
        unencoded: &[
            165, 90, 73, 153, 181, 8, 211, 6, 177, 46, 9, 212, 45, 11, 27, 28,
        ],
        encoded: &"wiprugpibdjopcjqb8kn4na5do",
        bits: 126,
    },
    TestCase {
        unencoded: &[109, 141, 150, 155, 20, 184],
        encoded: &"psg3pgawzy",
        bits: 46,
    },
    TestCase {
        unencoded: &[211, 162, 45, 126, 107, 174, 173, 76, 96, 241, 233, 236, 176],
        encoded: &"4qtn49umi4swaa8t78smy",
        bits: 101,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[21, 218, 130, 241, 13, 22, 255, 152, 37, 5, 132],
        encoded: &"nzpefhepn593ojefoo",
        bits: 87,
    },
    TestCase {
        unencoded: &[
            140, 22, 76, 54, 85, 216, 103, 251, 228, 246, 55, 242, 92, 225, 177, 128,
        ],
        encoded: &"tomrap1i5bu9z38sg93f3apto",
        bits: 121,
    },
    TestCase {
        unencoded: &[120, 90, 254],
        encoded: &"xbpxh",
        bits: 24,
    },
    TestCase {
        unencoded: &[162, 23, 150, 42, 64, 7, 217, 149, 183, 202, 180, 192],
        encoded: &"wem3ck1yy9c3mp6ksuy",
        bits: 92,
    },
    TestCase {
        unencoded: &[142, 175, 64, 61, 58, 0],
        encoded: &"t4zwyxj4y",
        bits: 41,
    },
    TestCase {
        unencoded: &[
            54, 176, 233, 47, 27, 194, 240, 117, 188, 91, 240, 126, 192, 160,
        ],
        encoded: &"g4aq1ma5ama8mxn56b9cbe",
        bits: 108,
    },
    TestCase {
        unencoded: &[3, 41, 21, 207, 108],
        encoded: &"ycwtmu5c",
        bits: 39,
    },
    TestCase {
        unencoded: &[
            92, 13, 77, 130, 30, 160, 149, 193, 98, 169, 71, 88, 204, 233, 7, 0,
        ],
        encoded: &"mogw5yo6wnkhnaije7cc34e8y",
        bits: 122,
    },
    TestCase {
        unencoded: &[204, 71, 106, 40, 214, 120, 254, 129, 64],
        encoded: &"3tdswkgsxd9eno",
        bits: 70,
    },
    TestCase {
        unencoded: &[178, 95, 225, 126, 44, 137, 97, 145, 240, 90, 33, 224, 0],
        encoded: &"sjx6n9tctfo3dhn4r8oy",
        bits: 97,
    },
    TestCase {
        unencoded: &[29, 145, 80, 223, 189, 79, 184, 128],
        encoded: &"dseibz77j6he",
        bits: 57,
    },
    TestCase {
        unencoded: &[
            19, 167, 57, 197, 50, 221, 80, 189, 85, 8, 175, 179, 88, 105, 64,
        ],
        encoded: &"nquuutj15iem4ieei63io4k",
        bits: 114,
    },
    TestCase {
        unencoded: &[67, 33, 26, 98, 176],
        encoded: &"ecotwaio",
        bits: 40,
    },
    TestCase {
        unencoded: &[59, 133, 131, 126, 55, 235, 48],
        encoded: &"8qnag9tz7ca",
        bits: 54,
    },
    TestCase {
        unencoded: &[45, 22, 106, 70, 23, 179],
        encoded: &"fwmgwtozsc",
        bits: 48,
    },
    TestCase {
        unencoded: &[
            13, 214, 229, 241, 41, 235, 96, 152, 12, 68, 16, 160, 242, 149, 192,
        ],
        encoded: &"bzmqmhjj7pojodnrnnoxffq",
        bits: 114,
    },
    TestCase {
        unencoded: &[22, 96, 162, 39, 64],
        encoded: &"n3okrj4",
        bits: 34,
    },
    TestCase {
        unencoded: &[
            108, 49, 28, 42, 193, 223, 247, 7, 101, 118, 223, 77, 226, 181, 251, 128,
        ],
        encoded: &"poataksb595oq3ms57g6fpx5o",
        bits: 121,
    },
    TestCase {
        unencoded: &[125, 241, 190, 89, 49, 58, 100, 137, 181, 53, 34, 63, 168],
        encoded: &"xza5hsjt8j1eupjire94o",
        bits: 102,
    },
    TestCase {
        unencoded: &[0, 200, 0, 52, 168, 149, 231, 140, 224],
        encoded: &"ydryypfe1zua3a",
        bits: 68,
    },
    TestCase {
        unencoded: &[141, 45, 199, 169, 110, 30, 105, 149, 56],
        encoded: &"twshxkmqd3w3kq",
        bits: 69,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 1,
    },
    TestCase {
        unencoded: &[82, 57, 148],
        encoded: &"keh3e",
        bits: 22,
    },
    TestCase {
        unencoded: &[196, 28, 107, 77, 172, 102, 101, 184],
        encoded: &"aoqgsupcc315o",
        bits: 61,
    },
    TestCase {
        unencoded: &[251, 248, 217, 123, 91, 123, 10, 186, 181, 201, 137],
        encoded: &"9xhp1645xcfmipqjtr",
        bits: 88,
    },
    TestCase {
        unencoded: &[150, 126, 143, 243, 34, 206, 171, 70, 233, 62],
        encoded: &"139e9h3n34iwp4j6",
        bits: 79,
    },
    TestCase {
        unencoded: &[
            116, 242, 90, 196, 55, 114, 48, 253, 103, 172, 161, 34, 66, 192,
        ],
        encoded: &"qu3fitbzqeax437cwrtrfo",
        bits: 108,
    },
    TestCase {
        unencoded: &[30, 88, 89, 8, 240, 104, 57, 209, 214, 176, 236, 128],
        encoded: &"d3cf1n8opyh7diio71",
        bits: 89,
    },
    TestCase {
        unencoded: &[10, 221, 60, 59, 158, 188, 222, 0],
        encoded: &"bmquaqh6zuxyy",
        bits: 62,
    },
    TestCase {
        unencoded: &[123, 55, 82, 170, 129, 23, 198, 219, 86, 253, 126, 90, 142],
        encoded: &"xc5ifkwbn9dpsiz7x3peh",
        bits: 103,
    },
    TestCase {
        unencoded: &[220, 154, 117, 31, 188, 161, 243, 3, 42, 27, 20, 172, 96],
        encoded: &"51p8k87hw83ogko5n1sg",
        bits: 99,
    },
    TestCase {
        unencoded: &[40],
        encoded: &"fy",
        bits: 6,
    },
    TestCase {
        unencoded: &[
            24, 240, 22, 94, 223, 105, 68, 167, 11, 79, 223, 252, 114, 109, 154, 0,
        ],
        encoded: &"ddabczs9pfnkqn4x5968r5c4y",
        bits: 121,
    },
    TestCase {
        unencoded: &[
            129, 89, 196, 9, 141, 134, 202, 44, 182, 36, 37, 200, 177, 21, 50,
        ],
        encoded: &"ofchencpo5fn3ptrrzrmnfj1",
        bits: 119,
    },
    TestCase {
        unencoded: &[
            134, 141, 197, 52, 187, 24, 76, 11, 193, 241, 200, 89, 197, 219, 96,
        ],
        encoded: &"o4ghkpf5dbgyzoxt3bchms5y",
        bits: 116,
    },
    TestCase {
        unencoded: &[98, 186, 168, 78, 143, 29, 144],
        encoded: &"ck7kouwxdse",
        bits: 52,
    },
    TestCase {
        unencoded: &[60, 48, 188],
        encoded: &"8oama",
        bits: 22,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 5,
    },
    TestCase {
        unencoded: &[126, 165, 7, 8, 158, 128, 144, 104, 67, 231, 241],
        encoded: &"x41oqnr6onegoo986r",
        bits: 88,
    },
    TestCase {
        unencoded: &[148, 122, 116, 101, 39, 64],
        encoded: &"1t78e3j8e",
        bits: 42,
    },
    TestCase {
        unencoded: &[146, 43, 165, 66, 253, 76, 158, 128],
        encoded: &"1ei4koz7j1xe",
        bits: 60,
    },
    TestCase {
        unencoded: &[33, 193, 170, 170, 100],
        encoded: &"r8y4ikur",
        bits: 39,
    },
    TestCase {
        unencoded: &[211, 86, 208],
        encoded: &"4pmp",
        bits: 20,
    },
    TestCase {
        unencoded: &[252, 146, 94, 63, 52, 117, 165, 125, 172, 128],
        encoded: &"91jfhx3wqs1z5mr",
        bits: 73,
    },
    TestCase {
        unencoded: &[251, 187, 210, 29, 72, 122, 47, 17, 81, 255, 52, 18, 204, 0],
        encoded: &"9q77r8kexeztnwx9gojca",
        bits: 105,
    },
    TestCase {
        unencoded: &[187, 196, 78, 233, 97, 213, 1, 22, 132, 30, 236, 32, 0],
        encoded: &"zxnr74mb4wytpby67ooy",
        bits: 97,
    },
    TestCase {
        unencoded: &[
            98, 177, 20, 66, 107, 26, 103, 89, 35, 160, 138, 184, 225, 245, 0,
        ],
        encoded: &"ckateoumdjui1e7ytkhqd7e",
        bits: 114,
    },
    TestCase {
        unencoded: &[242, 243, 143, 19, 46],
        encoded: &"6m3a6r3q",
        bits: 39,
    },
    TestCase {
        unencoded: &[95, 57, 65, 192],
        encoded: &"mhhwdo",
        bits: 27,
    },
    TestCase {
        unencoded: &[185, 125, 158, 40, 56, 129, 161, 199, 46, 17, 160, 100],
        encoded: &"zf63hkbaogohqmotwb1",
        bits: 94,
    },
    TestCase {
        unencoded: &[141, 129, 150, 211, 225, 53, 0],
        encoded: &"tsy3pw9bgw",
        bits: 49,
    },
    TestCase {
        unencoded: &[185, 57, 200],
        encoded: &"zrhho",
        bits: 21,
    },
    TestCase {
        unencoded: &[
            12, 71, 75, 195, 102, 7, 170, 121, 241, 204, 37, 4, 103, 185, 156,
        ],
        encoded: &"btdwzo5gy6i8uhqcrwngxqch",
        bits: 118,
    },
    TestCase {
        unencoded: &[18, 132, 35, 222, 155, 18, 169, 115, 254, 34, 83, 174, 53],
        encoded: &"nknn8zw5nkwz89tnkqzdk",
        bits: 104,
    },
    TestCase {
        unencoded: &[72, 50, 179, 40, 55, 230, 37, 160],
        encoded: &"jy3mgkbzha14y",
        bits: 62,
    },
    TestCase {
        unencoded: &[147, 215, 163, 45, 121, 211, 50, 22, 240],
        encoded: &"1xm4gmm34c3bph",
        bits: 70,
    },
    TestCase {
        unencoded: &[
            121, 143, 234, 210, 170, 137, 119, 210, 215, 109, 253, 255, 160, 134, 225, 192,
        ],
        encoded: &"xg86iwiktf57fi5p9z94bbzba",
        bits: 122,
    },
    TestCase {
        unencoded: &[
            77, 96, 227, 108, 184, 127, 126, 189, 218, 30, 112, 202, 162, 234, 212,
        ],
        encoded: &"jioqg5fax79m5so6qdfkf4sw",
        bits: 120,
    },
    TestCase {
        unencoded: &[75, 166, 121, 34, 245, 76, 179, 2],
        encoded: &"jqu81ezij13or",
        bits: 63,
    },
    TestCase {
        unencoded: &[212, 204, 177, 157, 62, 189, 39, 238, 150, 213, 116],
        encoded: &"4ugmd8j6zwu67fsiqo",
        bits: 88,
    },
    TestCase {
        unencoded: &[159, 65, 228, 69, 13, 247, 196, 202, 242, 238, 5, 204, 0],
        encoded: &"u7y6etep69ncihzqyzgy",
        bits: 97,
    },
    TestCase {
        unencoded: &[
            242, 197, 42, 225, 225, 171, 164, 204, 141, 53, 60, 66, 11, 122, 147, 126,
        ],
        encoded: &"6mn1iaxbiq1c3dji8tbys6wuxa",
        bits: 127,
    },
    TestCase {
        unencoded: &[223, 3, 200, 55, 203, 163, 115, 190, 132],
        encoded: &"5hbhop6mwp357b",
        bits: 70,
    },
    TestCase {
        unencoded: &[156, 156, 161, 176, 224],
        encoded: &"u1qkdc8y",
        bits: 39,
    },
    TestCase {
        unencoded: &[230, 64],
        encoded: &"h3",
        bits: 10,
    },
    TestCase {
        unencoded: &[205, 101, 212],
        encoded: &"3i17e",
        bits: 22,
    },
    TestCase {
        unencoded: &[37, 20, 170, 114, 194, 144, 216, 175, 187, 254, 168, 66, 48],
        encoded: &"rwkkwhsn1dck9q96ibbd",
        bits: 100,
    },
    TestCase {
        unencoded: &[
            224, 52, 23, 237, 123, 15, 199, 19, 146, 102, 23, 150, 141, 92, 152, 160,
        ],
        encoded: &"hy4bx5m5b9dt8rugn6me4zraw",
        bits: 124,
    },
    TestCase {
        unencoded: &[243, 24, 146, 99, 32, 121, 240],
        encoded: &"6ccjra3yx8a",
        bits: 54,
    },
    TestCase {
        unencoded: &[171, 104, 192, 29, 240, 197, 144],
        encoded: &"ipwcy8xoase",
        bits: 53,
    },
    TestCase {
        unencoded: &[94, 186, 203, 190, 170, 175, 63, 168],
        encoded: &"m47czxikih94o",
        bits: 62,
    },
    TestCase {
        unencoded: &[75, 0],
        encoded: &"jc",
        bits: 9,
    },
    TestCase {
        unencoded: &[189, 25, 142, 166, 181, 50, 47, 251, 37, 192, 243, 17, 8, 64],
        encoded: &"zwca7jiigez9sjqy6ceooo",
        bits: 108,
    },
    TestCase {
        unencoded: &[
            44, 210, 48, 172, 73, 218, 94, 91, 67, 254, 168, 214, 106, 97, 128,
        ],
        encoded: &"fujdbmnj5jxfso96idmgwac",
        bits: 115,
    },
    TestCase {
        unencoded: &[124, 16, 75, 38, 94, 180, 83, 207, 89, 255, 92, 64],
        encoded: &"xoersj16stjh6sx9mt",
        bits: 90,
    },
    TestCase {
        unencoded: &[60, 138, 201, 172, 251, 185, 187, 249, 55, 182, 115, 0],
        encoded: &"81fcum85zg791p7sqc",
        bits: 90,
    },
    TestCase {
        unencoded: &[0, 120, 0],
        encoded: &"ybhy",
        bits: 17,
    },
    TestCase {
        unencoded: &[112],
        encoded: &"q",
        bits: 4,
    },
    TestCase {
        unencoded: &[216, 86, 97, 211, 55, 232, 115, 23, 181, 21, 21, 0],
        encoded: &"5bmgdw3z7b3txpeinw",
        bits: 89,
    },
    TestCase {
        unencoded: &[145, 36],
        encoded: &"1r1",
        bits: 14,
    },
    TestCase {
        unencoded: &[136, 242, 168, 186, 196, 32],
        encoded: &"td3ktqsrry",
        bits: 47,
    },
    TestCase {
        unencoded: &[2, 104, 197, 222, 219, 87, 126, 117, 45, 128],
        encoded: &"yjwcmzs5k798kmc",
        bits: 73,
    },
    TestCase {
        unencoded: &[79, 106, 224, 247, 216, 64, 224],
        encoded: &"j7iqb76aedo",
        bits: 51,
    },
    TestCase {
        unencoded: &[18, 199, 50, 177, 123, 101, 239, 250, 208],
        encoded: &"nmdufcm5czz9iw",
        bits: 69,
    },
    TestCase {
        unencoded: &[224],
        encoded: &"h",
        bits: 4,
    },
    TestCase {
        unencoded: &[
            87, 38, 96, 237, 205, 195, 53, 244, 105, 45, 231, 222, 95, 44,
        ],
        encoded: &"khugb5qpac49e4jph9xf6my",
        bits: 112,
    },
    TestCase {
        unencoded: &[73, 219, 247, 152, 150, 5, 60, 135, 7, 2, 224],
        encoded: &"j8p9xgrsyw6eqbanh",
        bits: 83,
    },
    TestCase {
        unencoded: &[41, 240, 146, 61, 254, 205, 217, 128],
        encoded: &"f8ajrxx63zca",
        bits: 57,
    },
    TestCase {
        unencoded: &[174, 183, 124, 147, 199, 250],
        encoded: &"i45z3r689e",
        bits: 47,
    },
    TestCase {
        unencoded: &[86, 117, 200, 146, 212, 72, 17, 174],
        encoded: &"k34htrswjye4h",
        bits: 64,
    },
    TestCase {
        unencoded: &[30, 168, 148, 95, 51, 193, 42, 58, 139, 241, 197, 174, 192],
        encoded: &"d4wjez3uaridin9taszcy",
        bits: 101,
    },
    TestCase {
        unencoded: &[203, 60, 145, 18, 144],
        encoded: &"3c6jnrwo",
        bits: 36,
    },
    TestCase {
        unencoded: &[96],
        encoded: &"c",
        bits: 3,
    },
    TestCase {
        unencoded: &[182, 142, 115, 255, 82, 215],
        encoded: &"s48889414h",
        bits: 48,
    },
    TestCase {
        unencoded: &[37, 192],
        encoded: &"rzy",
        bits: 12,
    },
    TestCase {
        unencoded: &[54, 171, 56],
        encoded: &"g4iuo",
        bits: 21,
    },
    TestCase {
        unencoded: &[228],
        encoded: &"ho",
        bits: 7,
    },
    TestCase {
        unencoded: &[245, 103, 223, 175, 218, 41, 150, 105, 32],
        encoded: &"6iu79m64fgmg1e",
        bits: 67,
    },
    TestCase {
        unencoded: &[121, 23, 162, 222, 135, 24, 16, 198, 171, 119, 98, 0],
        encoded: &"xrm4fzw8dyecpk5zce",
        bits: 90,
    },
    TestCase {
        unencoded: &[230, 141, 54, 213, 76, 150, 222, 112],
        encoded: &"h4gupikc15x8y",
        bits: 61,
    },
    TestCase {
        unencoded: &[54, 35, 25, 140, 93, 60, 131, 204, 166, 155, 20, 7, 188, 246],
        encoded: &"gattudn781bh3jw5nod537o",
        bits: 112,
    },
    TestCase {
        unencoded: &[
            144, 65, 25, 203, 95, 41, 214, 215, 139, 79, 116, 43, 55, 75, 189, 208,
        ],
        encoded: &"1bytu149f8mpxn4xqoiuq1774",
        bits: 125,
    },
    TestCase {
        unencoded: &[130, 89, 238, 41, 153, 76, 150, 0],
        encoded: &"ojc6hkc3j1my",
        bits: 57,
    },
    TestCase {
        unencoded: &[2, 93, 147, 48, 189, 24, 64],
        encoded: &"yjq3gcf7dby",
        bits: 55,
    },
    TestCase {
        unencoded: &[149, 46, 110, 8, 205, 140, 98, 139, 148, 0],
        encoded: &"1wzghngptttezfyy",
        bits: 77,
    },
    TestCase {
        unencoded: &[175, 38, 57, 78, 230, 153, 89, 130, 224],
        encoded: &"ihud1uzgufcafa",
        bits: 67,
    },
    TestCase {
        unencoded: &[
            59, 111, 234, 167, 185, 147, 159, 69, 98, 197, 193, 234, 159, 32,
        ],
        encoded: &"8pz6ij731qxwkasfa8ij6e",
        bits: 107,
    },
    TestCase {
        unencoded: &[160, 112, 48],
        encoded: &"wbad",
        bits: 20,
    },
    TestCase {
        unencoded: &[85, 33, 200, 52, 45, 27, 226, 73, 156, 4, 178],
        encoded: &"kwohopbpdxtru8yrse",
        bits: 88,
    },
    TestCase {
        unencoded: &[49, 31, 22],
        encoded: &"grxtc",
        bits: 24,
    },
    TestCase {
        unencoded: &[75, 29, 119, 156, 192],
        encoded: &"jcqzx8g",
        bits: 34,
    },
    TestCase {
        unencoded: &[206, 114, 211, 78, 160, 84, 31, 202, 170, 171, 122, 18, 0],
        encoded: &"333pguiykoxhikimxejy",
        bits: 99,
    },
    TestCase {
        unencoded: &[251, 214, 59, 127, 8, 248, 177, 22, 213, 170],
        encoded: &"9xmds9ae9natpipk",
        bits: 79,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[32, 245, 98],
        encoded: &"rd4sr",
        bits: 23,
    },
    TestCase {
        unencoded: &[0],
        encoded: &"y",
        bits: 1,
    },
    TestCase {
        unencoded: &[189, 146, 12, 132, 59, 32],
        encoded: &"zsjy3bb5r",
        bits: 43,
    },
    TestCase {
        unencoded: &[128],
        encoded: &"o",
        bits: 3,
    },
    TestCase {
        unencoded: &[114, 198, 242, 242, 232, 80, 199, 176, 79, 50, 194, 222, 0],
        encoded: &"qmdxfhzekdd5yu31amxy",
        bits: 99,
    },
    TestCase {
        unencoded: &[
            48, 236, 244, 252, 52, 51, 227, 25, 172, 228, 122, 192, 38, 120,
        ],
        encoded: &"gdsxj9bwgxttum8rxmync6y",
        bits: 111,
    },
    TestCase {
        unencoded: &[16],
        encoded: &"ny",
        bits: 6,
    },
    TestCase {
        unencoded: &[72, 61, 134, 211, 88],
        encoded: &"jy6apw4a",
        bits: 37,
    },
    TestCase {
        unencoded: &[131, 115, 227, 209, 134, 40],
        encoded: &"op368wcgfy",
        bits: 46,
    },
    TestCase {
        unencoded: &[97, 244],
        encoded: &"c84y",
        bits: 16,
    },
    TestCase {
        unencoded: &[34, 15, 99, 61, 16, 43, 253, 162, 212, 0],
        encoded: &"re8sgxeofx64fiy",
        bits: 73,
    },
    TestCase {
        unencoded: &[170, 10, 222, 128],
        encoded: &"iefp7",
        bits: 25,
    },
    TestCase {
        unencoded: &[158, 213, 126, 154, 88],
        encoded: &"u5kz7g1a",
        bits: 37,
    },
    TestCase {
        unencoded: &[
            160, 124, 200, 127, 202, 95, 176, 19, 72, 105, 42, 99, 125, 60,
        ],
        encoded: &"wb6co96km6abg1djfjtz4x",
        bits: 110,
    },
    TestCase {
        unencoded: &[40],
        encoded: &"fy",
        bits: 7,
    },
    TestCase {
        unencoded: &[248, 55, 182, 233, 216, 38, 60, 176, 192],
        encoded: &"9y55p4qara6mbo",
        bits: 69,
    },
    TestCase {
        unencoded: &[251, 228, 170, 35, 222, 101, 64, 75, 22, 240],
        encoded: &"9x1kwe66ciyrsfzo",
        bits: 76,
    },
    TestCase {
        unencoded: &[32],
        encoded: &"r",
        bits: 3,
    },
    TestCase {
        unencoded: &[172, 71, 165, 65, 8, 183, 223, 246, 36, 105, 238, 24],
        encoded: &"itd4koees9x9cjdj7ac",
        bits: 95,
    },
    TestCase {
        unencoded: &[236, 126, 11, 74, 118, 47, 45, 145, 128],
        encoded: &"7t9ys1usfhs3d",
        bits: 65,
    },
    TestCase {
        unencoded: &[247],
        encoded: &"6h",
        bits: 8,
    },
    TestCase {
        unencoded: &[251, 80, 214, 189, 128],
        encoded: &"9peppxc",
        bits: 33,
    },
    TestCase {
        unencoded: &[
            12, 174, 128, 59, 14, 240, 43, 168, 245, 225, 163, 26, 248, 192,
        ],
        encoded: &"b1zeyqaq6yi4t7xbwcpxto",
        bits: 106,
    },
    TestCase {
        unencoded: &[100, 177, 133, 66, 229, 47, 73, 6, 234, 172, 230, 144, 192],
        encoded: &"c1aakozff7rop4ich4ec",
        bits: 98,
    },
    TestCase {
        unencoded: &[49, 107, 121, 225, 95, 103, 252, 129, 204],
        encoded: &"gfizuak9c96edu",
        bits: 70,
    },
    TestCase {
        unencoded: &[152, 97, 240, 55, 252, 92],
        encoded: &"ubo9yp9hmo",
        bits: 46,
    },
    TestCase {
        unencoded: &[
            176, 81, 48, 88, 232, 32, 160, 247, 167, 42, 136, 33, 225, 220, 128,
        ],
        encoded: &"sbeuys8ernoxxj3ktyo6dzr",
        bits: 113,
    },
    TestCase {
        unencoded: &[
            250, 231, 62, 38, 252, 76, 143, 163, 187, 114, 33, 126, 155, 184, 112,
        ],
        encoded: &"9muuhjzhj1848q51rf9jzqdo",
        bits: 118,
    },
    TestCase {
        unencoded: &[
            121, 62, 110, 24, 108, 232, 33, 119, 245, 38, 202, 249, 179, 0,
        ],
        encoded: &"xr9ghgdc7yozx7jg3mh5gy",
        bits: 106,
    },
    TestCase {
        unencoded: &[220, 215, 156, 245, 153, 5],
        encoded: &"5um337c3yw",
        bits: 48,
    },
    TestCase {
        unencoded: &[81, 111, 54],
        encoded: &"kfzuc",
        bits: 24,
    },
    TestCase {
        unencoded: &[151, 155, 96],
        encoded: &"16ps",
        bits: 19,
    },
    TestCase {
        unencoded: &[241, 92, 195, 212, 245],
        encoded: &"6fqc8i8i",
        bits: 40,
    },
    TestCase {
        unencoded: &[174, 128],
        encoded: &"i4",
        bits: 9,
    },
    TestCase {
        unencoded: &[39, 203, 69, 1, 37],
        encoded: &"r9fwkyjf",
        bits: 40,
    },
    TestCase {
        unencoded: &[
            19, 110, 70, 123, 239, 68, 70, 40, 188, 206, 215, 195, 205, 180, 57, 104,
        ],
        encoded: &"npzrc69xetdntxgq49bh5pb3py",
        bits: 127,
    },
    TestCase {
        unencoded: &[80, 227, 135, 16, 33, 197, 86, 155, 85, 112, 0, 64],
        encoded: &"kdtaqrbbaimjsimoyb",
        bits: 90,
    },
    TestCase {
        unencoded: &[51, 140, 9, 92, 121, 183, 155, 246, 216, 158, 56],
        encoded: &"gqgy1zd3s6p9psr68",
        bits: 85,
    },
    TestCase {
        unencoded: &[244, 27, 120, 73, 128],
        encoded: &"6opzo1c",
        bits: 33,
    },
    TestCase {
        unencoded: &[117, 92, 191, 70, 194, 10, 220, 188, 47, 153, 245, 159],
        encoded: &"qiqm6tsnbmqmamh36sxo",
        bits: 96,
    },
    TestCase {
        unencoded: &[185, 204, 25, 77, 98, 238, 129, 142, 197, 0],
        encoded: &"z8gb1umn74ya7te",
        bits: 75,
    },
    TestCase {
        unencoded: &[245],
        encoded: &"6w",
        bits: 8,
    },
    TestCase {
        unencoded: &[254, 0],
        encoded: &"9ay",
        bits: 11,
    },
    TestCase {
        unencoded: &[146, 87, 67, 163, 58, 34, 118, 235, 187, 191, 136],
        encoded: &"1jmw8e34rj5qzq79t",
        bits: 85,
    },
    TestCase {
        unencoded: &[231, 177, 0],
        encoded: &"h6ao",
        bits: 18,
    },
    TestCase {
        unencoded: &[
            77, 65, 3, 169, 33, 234, 255, 54, 21, 130, 66, 121, 203, 136, 115, 0,
        ],
        encoded: &"jiyo8kjb7m9ucfcnejhhznduy",
        bits: 121,
    },
    TestCase {
        unencoded: &[160, 102, 78, 74, 137, 96, 138, 146, 116, 171, 183, 228],
        encoded: &"wburh1wjcnfjr7fms91y",
        bits: 96,
    },
    TestCase {
        unencoded: &[210, 203],
        encoded: &"4mfo",
        bits: 16,
    },
    TestCase {
        unencoded: &[232, 174, 33, 251, 28, 193, 246, 124],
        encoded: &"7nznd6aha858a",
        bits: 63,
    },
    TestCase {
        unencoded: &[
            21, 109, 107, 49, 248, 96, 199, 8, 243, 140, 121, 8, 226, 236, 184, 228,
        ],
        encoded: &"nissscxacddothhcxrrqf5faho",
        bits: 126,
    },
    TestCase {
        unencoded: &[171, 199, 202, 42, 129, 91, 120, 96],
        encoded: &"ixdhwkwbmphg",
        bits: 59,
    },
    TestCase {
        unencoded: &[197, 252, 221, 19, 43, 59, 26, 137, 172, 192],
        encoded: &"az6p4r3m8cpeumgy",
        bits: 77,
    },
    TestCase {
        unencoded: &[
            18, 247, 219, 241, 39, 129, 175, 29, 43, 180, 71, 126, 58, 64, 20, 0,
        ],
        encoded: &"nm57zhj8ogzt4k7we79dwoywy",
        bits: 123,
    },
    TestCase {
        unencoded: &[95, 158, 4, 202, 25, 186, 202, 191, 121, 45, 4, 120, 89],
        encoded: &"m6xyj1o3zmfm66jpythf1",
        bits: 104,
    },
    TestCase {
        unencoded: &[146, 24, 34, 0],
        encoded: &"1ecnry",
        bits: 26,
    },
    TestCase {
        unencoded: &[69],
        encoded: &"ew",
        bits: 8,
    },
    TestCase {
        unencoded: &[49, 199, 228, 92, 14, 126, 184, 176],
        encoded: &"g8d6ezyqx4hm",
        bits: 60,
    },
    TestCase {
        unencoded: &[173, 251, 221, 71, 96],
        encoded: &"iz774t5",
        bits: 35,
    },
    TestCase {
        unencoded: &[
            198, 138, 137, 15, 38, 96, 35, 20, 51, 77, 68, 36, 153, 222, 84,
        ],
        encoded: &"a4fe1d3gcyttec4peo1juz1w",
        bits: 118,
    },
    TestCase {
        unencoded: &[144, 238, 21, 140, 30, 222, 117, 128],
        encoded: &"1dzbmdy6534a",
        bits: 57,
    },
    TestCase {
        unencoded: &[61, 240, 128],
        encoded: &"8zae",
        bits: 17,
    },
    TestCase {
        unencoded: &[140, 238, 12, 14, 215, 209, 4],
        encoded: &"tuzyadsz4rny",
        bits: 56,
    },
    TestCase {
        unencoded: &[249, 155, 92, 96],
        encoded: &"9gpiaa",
        bits: 28,
    },
    TestCase {
        unencoded: &[96, 220, 33, 38, 173, 249, 187, 128, 65, 145, 253, 204, 8],
        encoded: &"cdqnnjip9g7ayoct9zgyo",
        bits: 101,
    },
    TestCase {
        unencoded: &[128, 67, 40, 87, 211, 24, 80, 128, 18, 62, 71, 151, 187, 0],
        encoded: &"obb1oi6udbeeyrt6e6m5s",
        bits: 105,
    },
    TestCase {
        unencoded: &[219, 86, 169, 239, 16],
        encoded: &"5pmku5ao",
        bits: 36,
    },
    TestCase {
        unencoded: &[17, 105, 16, 219, 57, 231, 26, 142, 96],
        encoded: &"nfwtbs33hhpeha",
        bits: 67,
    },
    TestCase {
        unencoded: &[33, 27, 69, 224],
        encoded: &"rrpwma",
        bits: 29,
    },
    TestCase {
        unencoded: &[166, 0],
        encoded: &"way",
        bits: 13,
    },
    TestCase {
        unencoded: &[
            238, 25, 116, 170, 30, 46, 206, 216, 61, 148, 17, 11, 16, 30, 64,
        ],
        encoded: &"7aczjko6f58poxcwnrfty81",
        bits: 114,
    },
    TestCase {
        unencoded: &[206, 184, 111],
        encoded: &"34hg6",
        bits: 24,
    },
    TestCase {
        unencoded: &[164, 171, 90, 16, 193, 206],
        encoded: &"w1iiwrgb3a",
        bits: 47,
    },
    TestCase {
        unencoded: &[95, 139, 101, 173, 89, 192],
        encoded: &"m6fsmmk3a",
        bits: 42,
    },
    TestCase {
        unencoded: &[163, 24, 200, 174, 225, 108, 241, 231, 144, 163, 240],
        encoded: &"wccctmzbpua6xrfd6",
        bits: 85,
    },
    TestCase {
        unencoded: &[249, 190, 155, 95, 58, 127, 31],
        encoded: &"9g9jsz34xhxo",
        bits: 56,
    },
    TestCase {
        unencoded: &[58, 20, 42, 176, 128],
        encoded: &"8eknicr",
        bits: 33,
    },
    TestCase {
        unencoded: &[
            142, 111, 48, 73, 104, 206, 154, 61, 213, 57, 188, 23, 107, 77, 158, 176,
        ],
        encoded: &"t3zuy1me34pd5ij3zomssuc6s",
        bits: 125,
    },
    TestCase {
        unencoded: &[134, 165, 37, 250, 8, 144],
        encoded: &"o411m6oe1",
        bits: 45,
    },
    TestCase {
        unencoded: &[
            161, 89, 90, 90, 206, 176, 159, 146, 248, 231, 52, 189, 216, 144, 192,
        ],
        encoded: &"wfciwssqsnx3f688g167trg",
        bits: 114,
    },
    TestCase {
        unencoded: &[149, 31, 20, 70, 254, 14, 165, 39, 64],
        encoded: &"1wxtetz6b411qo",
        bits: 69,
    },
    TestCase {
        unencoded: &[89, 47, 229, 119, 83, 231, 234, 128],
        encoded: &"mrz6k74uh9ie",
        bits: 58,
    },
    TestCase {
        unencoded: &[20, 231, 222, 113, 130, 7, 240, 124, 116, 51, 95, 0],
        encoded: &"nuu7hhcny9a8a7bumhy",
        bits: 91,
    },
    TestCase {
        unencoded: &[
            225, 164, 84, 248, 65, 140, 145, 238, 156, 181, 180, 233, 24, 227, 210,
        ],
        encoded: &"hg1fj6nbt1e678fisuwtta61",
        bits: 120,
    },
    TestCase {
        unencoded: &[28, 213, 217, 223, 184, 78, 56, 62, 207, 35, 216],
        encoded: &"duk7uz7ajahd7u3d5",
        bits: 85,
    },
    TestCase {
        unencoded: &[207, 225, 14, 177, 0],
        encoded: &"39oo7ce",
        bits: 33,
    },
    TestCase {
        unencoded: &[
            123, 28, 143, 147, 34, 96, 162, 239, 181, 58, 155, 41, 185, 194, 72,
        ],
        encoded: &"xcqe9r3ncntq9pj4ucw5uo1e",
        bits: 117,
    },
    TestCase {
        unencoded: &[65, 99, 247, 182, 212, 91, 42, 196, 159, 14, 144],
        encoded: &"eft9xpswmcicj8aq1",
        bits: 84,
    },
    TestCase {
        unencoded: &[60, 174, 219, 72, 145, 195, 56, 167, 168, 10, 165, 80],
        encoded: &"81zps1rtachkxkykwie",
        bits: 92,
    },
    TestCase {
        unencoded: &[214, 214, 17, 116, 36, 132, 13, 144],
        encoded: &"45mbn7broog3y",
        bits: 61,
    },
    TestCase {
        unencoded: &[30, 216, 14, 119, 239, 237],
        encoded: &"d5cyh79x7w",
        bits: 48,
    },
    TestCase {
        unencoded: &[34, 164, 166, 135, 143, 215, 48, 9, 140, 185, 116, 248, 159],
        encoded: &"rk1kpbhx4hayudf3quhj6",
        bits: 104,
    },
    TestCase {
        unencoded: &[98, 25, 43, 30, 157, 39, 75, 138, 24],
        encoded: &"cec1s8w7r7fawg",
        bits: 69,
    },
    TestCase {
        unencoded: &[131, 12, 9, 120, 250, 209, 28],
        encoded: &"ocgy16844rq",
        bits: 54,
    },
    TestCase {
        unencoded: &[252, 84, 169, 52, 181, 60, 133, 79, 186, 6, 186, 179, 128],
        encoded: &"9tkk1pfi81nw9qogzk3ay",
        bits: 101,
    },
    TestCase {
        unencoded: &[239, 70, 34, 140, 56, 238, 160],
        encoded: &"77dnfdba74o",
        bits: 51,
    },
    TestCase {
        unencoded: &[118, 58, 56],
        encoded: &"qa7do",
        bits: 21,
    },
    TestCase {
        unencoded: &[169, 118, 35, 90, 107],
        encoded: &"if5ngsum",
        bits: 40,
    },
    TestCase {
        unencoded: &[54, 30, 12],
        encoded: &"gaxya",
        bits: 23,
    },
    TestCase {
        unencoded: &[184, 252, 149, 166, 219, 212, 167, 10, 48, 134, 34, 57],
        encoded: &"zd6jmjs541uowcrgreho",
        bits: 96,
    },
    TestCase {
        unencoded: &[115, 185, 195, 130, 181, 0],
        encoded: &"qqhh8yiiy",
        bits: 42,
    },
    TestCase {
        unencoded: &[
            188, 6, 72, 205, 3, 66, 104, 134, 28, 153, 52, 4, 238, 16, 233, 160,
        ],
        encoded: &"zodrtuedejwec8r3gonqhr8jw",
        bits: 123,
    },
    TestCase {
        unencoded: &[148, 163, 219, 61, 122, 202],
        encoded: &"11t7sxm43e",
        bits: 47,
    },
    TestCase {
        unencoded: &[69, 240, 30, 240],
        encoded: &"ezab7h",
        bits: 28,
    },
    TestCase {
        unencoded: &[95, 162, 220, 104, 166, 181, 138, 171, 100, 228, 199, 8],
        encoded: &"m6tpa4fgssfks38rahr",
        bits: 94,
    },
    TestCase {
        unencoded: &[
            218, 38, 116, 231, 0, 126, 96, 147, 59, 82, 146, 231, 117, 45, 147, 92,
        ],
        encoded: &"5eu8j3ayx3ojgq411muzkmcumo",
        bits: 126,
    },
    TestCase {
        unencoded: &[218, 128, 235, 192],
        encoded: &"5kyqzo",
        bits: 27,
    },
    TestCase {
        unencoded: &[210, 108, 117, 142, 199, 131, 157, 40],
        encoded: &"4js8mds8oqq1o",
        bits: 62,
    },
    TestCase {
        unencoded: &[221, 110, 76, 90, 250, 128],
        encoded: &"5izrasz4o",
        bits: 43,
    },
    TestCase {
        unencoded: &[251, 141, 151, 115, 241, 96, 227, 230, 199, 22],
        encoded: &"9qg3qh9tcdt6ptas",
        bits: 80,
    },
    TestCase {
        unencoded: &[102, 31, 190, 0],
        encoded: &"cax5hy",
        bits: 28,
    },
    TestCase {
        unencoded: &[71, 183, 4, 14, 30, 216, 112],
        encoded: &"e65oedo65ba",
        bits: 52,
    },
    TestCase {
        unencoded: &[236, 39, 141, 165, 26, 248],
        encoded: &"7oua5je49y",
        bits: 46,
    },
    TestCase {
        unencoded: &[205, 100, 96, 207, 134, 219, 156, 36, 215, 127, 216, 192],
        encoded: &"3i1gbuhg5qqnji595d",
        bits: 90,
    },
    TestCase {
        unencoded: &[94, 93, 216, 129, 0, 57, 27, 48],
        encoded: &"m3q7tyey8rpu",
        bits: 60,
    },
    TestCase {
        unencoded: &[113, 38, 222, 246, 155, 7, 55, 77, 160, 30, 174, 131],
        encoded: &"qrup77w5yh5w5ey6i4bo",
        bits: 96,
    },
];
