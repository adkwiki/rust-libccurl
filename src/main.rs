/*
 * Copyright (c) 2016, Shinya Yagyu
 * All rights reserved.
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. Neither the name of the copyright holder nor the names of its
 *    contributors may be used to endorse or promote products derived from this
 *    software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

const TX_STRING: &str = "TISEZJUUKSTSX9KVQGXSYYLNDIBJDVRZSOFEHWJSDZLNUUNBDLHUODEGFZQTKOEXUMMQTOREUWQCSGGWRKALQDDZCQN9LBIEVKBFDCWBIDWD9DGVOJVCNUNWDDZFCIOICZZF9KIAYDCSKJWE99UPPLUQPUSWTDKTSSTJAQNYATUTXZPA9CCJRRNIRWXTAR9ECVYXC9AOHXHYVOS9LWDUOH9SDUAQBEYTMJIMUHJTGUSQTFPRLLXIDKOVZMONJHXPCD9FYLW9PN9LLPQBJRSEKVKKJB9JRTZCXSDBMJYAKDX99EGNLFZPKIADJQEIMCKRFQKIHGCJAHPL9JFJF9PHRKPCHBPN9LYQSC9TXOXAI9WBDIBNGFPLQS9BHTEVROMCAXXAXPVBAP9URJXIVZXIWWCMVDXGAFZOIRTJIMNIZEPGFMWXWOWRDUMHFRKL9LV9VJQIRZPVJSSKHXHHVZLRZYHGWQAVL9BMWKKFGZQEYJNCGROYYDIDULQVSXGVLTTZRLPSKPVIURJ9CJBTNAYCPHQTWTTKHXPABTYYCCVAZATEVED9PBJQTNOQEQQBTSATZJTVUTZPUWDYKROBROUVSPMDLUMEZWMPESEMQPSVTDZKATUTOAEVWCW9HIKKHMOQYJOUYLTFPERSKBVWARHGJNKUWGFZYF9WSTEHEQWCA9DTOTOTNDFGAEABKKBKEFLDELEOYPZTCVKOBIWA9HWTCQT9IGYVFAFAOLOJMRDZKCBYOCPGEGGZL9CGFURM9FJBLGLZJILNSFOBXLQOZWVLAZUFLGQNCAVJTBGVLZETETWGXLPSPWMMAEGORSDGPUSFRQ9AVWWZCFNKSAHIKJOMEWCCFGVYSDYNIXYYTKJTOKZUGLKNEXHWQ9HVFVJUGJJEDQACTWPSFOONTNCJRDQBSCGXVKWZIGDK9RGHKAHSTOJDJEHIAOF9MFLAZJXLUGQUAUGKQGQIXXNLAPRQNTNVDGXVZBSEFXVRR9ZQIZEWPXZFMXLJFTFKEPPAFJTMBLBWYAWJEIHUNATL9EHIJQTCCMQFHILGHGEVXKHDCNMAHDPUGBQYYBF9CRIKDVZZ9KIFELUUKPXPRIFVTZPXRBKJBRLEGUJKXZPYGXRKOAHROFXENAUAYOSQBJGMMHIDUNSYYGQSDJDKMPNBPTUWMIYZCWABYLDMTXAGWFYEXRGLOYVPNSOVYITEPCXMTMPVLBQPBNQUBITEM99KVRTPNAAWPR9RQYBLFZDVWYDJXQRGTVAFVE99KE9YSCETBIELIWPKZYFARSPVLTDKEAKLCKULZHLKOQZMVLFLF9QHT9LLS9QQODSFYUIPKSBVSKAJMVW9QUILQSKHZMAXGVHUJBMTATPIDHJVUBZWUOYNOOMEJVOUXHACUHDVKZ9ZDTSIHQOTOVUMEISMA9VZIFQTPBXXDHDLVLKZZHLYLPIE9SKOEJXAFDKICOYIOVVAEXC9VZSFSDTSHVEOSHIT9JHMBBPQTRGOREIYQSBCMHJQIXTTQWOCKMCSGBRTJRRYWPXAGELIFPG9YX9FNNYGSJXJYTHIMWSXZH9JQIYXKFXEOHOE9YNHJIDAJUGPENZHOIFEHBSCQITVFHUOESVXOJPCNTUZR9LVQCXYUW9DITEXPG9KWYMBZQQCESNFVUOBQGCRRKFHOEKTHDHUNRXADXUMCWFJMZTMHN9VWLZATB9FF9HBGLFITNNVFCQICPRSGVFAATWYJT9GUJIAHNNJBECYSWSGEJYLHJPUOYESLVIELBMSLRZJLPKDKFGAJSSWZCQDLFDEXWAPILHLNHKCRMPLQUYESAEIWWNBCEIYSOHKPILTXPAFIZ9JMKFKJHTLHRHGZQLCEVJJMJHWTUKMKOWTZWGVZGQAOAKVGXZEZBMYPVWUGYJBIFXBACZLADFFBZIXKWSZLDOCGRQAZDCFPRAZYXUMNRJ9UKUKRAVSVMCENDJABZITDQLNCXZNXCOHKLATFFXKP9FFDYSAXISISMVYPXPWYPVEAYRNAITWJSTGXRAMMZIZF9IUORREWSFUNZOXDVCMBZJAET9PVHCQTMDTVVXLXDIXFSHPXWKBZBDJAAXSDEFXPARBU9GJJABPMCD9LGQJLRIYKGQORGCDDABAIAQC9MZDQLXFSAOLNYMWCJODEEUSIHEVHQPAIFQL9ECBBVZPHYU9HDBOYXTKWOIRGHUJMVV9UKHHREDIU9CRZFUZKAMUVRIEMKEKIMAGXSMGTEJWCWWAMRPWNINTETOTRMODTORVEURRY9RTDYQIEW99999999999999999999999999999999999999999999CMRKHWD99A99999999C99999999TNFAKVBFHHMKQKKSNJRLDIYUIGOMEOADJLNS9JGKGUIHZHIUDNQMVYCA9SZCLQOEVJPUGQGWTMETLGMUQMAKHHHHTBHVWYSJSXRVBRMHVV9WUTNMNFVDWLHQGFELTKZOISREPUJXNRBIAQVQWCCKB9DEZEXS999999M9EZGRXJ9WYSZXNDZBAJZMJ9VAMUWWWANGIVFKCUNRB9GLZZKRIMEFUK9KEFZXYDGBQJIU9SQUM999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";

const HASH_LENGTH: usize = 243;
const STATE_LENGTH: usize = 3 * HASH_LENGTH;
const TRYTE_LENGTH: usize = HASH_LENGTH / 3; 

const TX_LENGTH: usize = 2673;
const TRITS_LENGTH: usize = 3 * TX_LENGTH;

fn main() {
    let trits = tx2trit(TX_STRING);
    let state = absorb(trits, TRITS_LENGTH);
    let _tryte = hash2tryte(state);
}

const CHAR_9_U8: u8 = '9' as u8;
const CHAR_A_U8: u8 = 'A' as u8;
const CHAR_Z_U8: u8 = 'Z' as u8;

const TRYTE2TRIT_TABLE: [[i8; 3]; 27] = [
    [0, 0, 0], [1, 0, 0], [-1, 1, 0], [0, 1, 0], [1, 1, 0], [-1, -1, 1], [0, -1, 1], [1, -1, 1], [-1, 0, 1], [0, 0, 1], [1, 0, 1], [-1, 1, 1], [0, 1, 1], [1, 1, 1], [-1, -1, -1], [0, -1, -1], [1, -1, -1], [-1, 0, -1], [0, 0, -1], [1, 0, -1], [-1, 1, -1], [0, 1, -1], [1, 1, -1], [-1, -1, 0], [0, -1, 0], [1, -1, 0], [-1, 0, 0]
];

const INDICES: [usize; 730] = [
    0, 364, 728, 363, 727, 362, 726, 361, 725, 360, 724, 359, 723, 358, 722, 357, 721, 356, 720, 355, 719, 354, 718, 353, 717, 352, 716, 351, 715, 350, 714, 349, 713, 348, 712, 347, 711, 346, 710, 345, 709, 344, 708, 343, 707, 342, 706, 341, 705, 340, 704, 339, 703, 338, 702, 337, 701, 336, 700, 335, 699, 334, 698, 333, 697, 332, 696, 331, 695, 330, 694, 329, 693, 328, 692, 327, 691, 326, 690, 325, 689, 324, 688, 323, 687, 322, 686, 321, 685, 320, 684, 319, 683, 318, 682, 317, 681, 316, 680, 315, 679, 314, 678, 313, 677, 312, 676, 311, 675, 310, 674, 309, 673, 308, 672, 307, 671, 306, 670, 305, 669, 304, 668, 303, 667, 302, 666, 301, 665, 300, 664, 299, 663, 298, 662, 297, 661, 296, 660, 295, 659, 294, 658, 293, 657, 292, 656, 291, 655, 290, 654, 289, 653, 288, 652, 287, 651, 286, 650, 285, 649, 284, 648, 283, 647, 282, 646, 281, 645, 280, 644, 279, 643, 278, 642, 277, 641, 276, 640, 275, 639, 274, 638, 273, 637, 272, 636, 271, 635, 270, 634, 269, 633, 268, 632, 267, 631, 266, 630, 265, 629, 264, 628, 263, 627, 262, 626, 261, 625, 260, 624, 259, 623, 258, 622, 257, 621, 256, 620, 255, 619, 254, 618, 253, 617, 252, 616, 251, 615, 250, 614, 249, 613, 248, 612, 247, 611, 246, 610, 245, 609, 244, 608, 243, 607, 242, 606, 241, 605, 240, 604, 239, 603, 238, 602, 237, 601, 236, 600, 235, 599, 234, 598, 233, 597, 232, 596, 231, 595, 230, 594, 229, 593, 228, 592, 227, 591, 226, 590, 225, 589, 224, 588, 223, 587, 222, 586, 221, 585, 220, 584, 219, 583, 218, 582, 217, 581, 216, 580, 215, 579, 214, 578, 213, 577, 212, 576, 211, 575, 210, 574, 209, 573, 208, 572, 207, 571, 206, 570, 205, 569, 204, 568, 203, 567, 202, 566, 201, 565, 200, 564, 199, 563, 198, 562, 197, 561, 196, 560, 195, 559, 194, 558, 193, 557, 192, 556, 191, 555, 190, 554, 189, 553, 188, 552, 187, 551, 186, 550, 185, 549, 184, 548, 183, 547, 182, 546, 181, 545, 180, 544, 179, 543, 178, 542, 177, 541, 176, 540, 175, 539, 174, 538, 173, 537, 172, 536, 171, 535, 170, 534, 169, 533, 168, 532, 167, 531, 166, 530, 165, 529, 164, 528, 163, 527, 162, 526, 161, 525, 160, 524, 159, 523, 158, 522, 157, 521, 156, 520, 155, 519, 154, 518, 153, 517, 152, 516, 151, 515, 150, 514, 149, 513, 148, 512, 147, 511, 146, 510, 145, 509, 144, 508, 143, 507, 142, 506, 141, 505, 140, 504, 139, 503, 138, 502, 137, 501, 136, 500, 135, 499, 134, 498, 133, 497, 132, 496, 131, 495, 130, 494, 129, 493, 128, 492, 127, 491, 126, 490, 125, 489, 124, 488, 123, 487, 122, 486, 121, 485, 120, 484, 119, 483, 118, 482, 117, 481, 116, 480, 115, 479, 114, 478, 113, 477, 112, 476, 111, 475, 110, 474, 109, 473, 108, 472, 107, 471, 106, 470, 105, 469, 104, 468, 103, 467, 102, 466, 101, 465, 100, 464, 99, 463, 98, 462, 97, 461, 96, 460, 95, 459, 94, 458, 93, 457, 92, 456, 91, 455, 90, 454, 89, 453, 88, 452, 87, 451, 86, 450, 85, 449, 84, 448, 83, 447, 82, 446, 81, 445, 80, 444, 79, 443, 78, 442, 77, 441, 76, 440, 75, 439, 74, 438, 73, 437, 72, 436, 71, 435, 70, 434, 69, 433, 68, 432, 67, 431, 66, 430, 65, 429, 64, 428, 63, 427, 62, 426, 61, 425, 60, 424, 59, 423, 58, 422, 57, 421, 56, 420, 55, 419, 54, 418, 53, 417, 52, 416, 51, 415, 50, 414, 49, 413, 48, 412, 47, 411, 46, 410, 45, 409, 44, 408, 43, 407, 42, 406, 41, 405, 40, 404, 39, 403, 38, 402, 37, 401, 36, 400, 35, 399, 34, 398, 33, 397, 32, 396, 31, 395, 30, 394, 29, 393, 28, 392, 27, 391, 26, 390, 25, 389, 24, 388, 23, 387, 22, 386, 21, 385, 20, 384, 19, 383, 18, 382, 17, 381, 16, 380, 15, 379, 14, 378, 13, 377, 12, 376, 11, 375, 10, 374, 9, 373, 8, 372, 7, 371, 6, 370, 5, 369, 4, 368, 3, 367, 2, 366, 1, 365, 0
];

const TRUTH_TABLE: [i8; 11] = [ 1, 0, -1, 0, 1, -1, 0, 0, -1, 1, 0 ];

fn tx2trit(tx: &str) -> [i8; TRITS_LENGTH] {
    println!("tx2trit");

    let tx_bytes = tx.as_bytes();
    if tx_bytes.len() != TX_LENGTH {
        panic!("tx len must be {} but {}", TX_LENGTH, tx_bytes.len());
    }

    let mut trits: [i8; TRITS_LENGTH] = [0; TRITS_LENGTH];
    for count in 0..TX_LENGTH {
        if !(tx_bytes[count] == CHAR_9_U8 || (tx_bytes[count] >= CHAR_A_U8 && tx_bytes[count] <= CHAR_Z_U8)) {
            panic!("illegal tx char {}", tx_bytes[count]);
        }

        // tx_bytes 9 -> 0, A -> 1, ... Z -> 34
        let mut ch: usize = 0;
        if tx_bytes[count] != CHAR_9_U8 {
            ch = (tx_bytes[count] - CHAR_A_U8 + 1) as usize;
        }

        trits[count * 3] = TRYTE2TRIT_TABLE[ch][0];
        trits[count * 3 + 1] = TRYTE2TRIT_TABLE[ch][1];
        trits[count * 3 + 2] = TRYTE2TRIT_TABLE[ch][2];
    }

    trits
}

fn transform(state: [i8; STATE_LENGTH]) -> [i8; STATE_LENGTH] {
    let mut transformed_state: [i8; STATE_LENGTH] = state;
    let mut round = 0;
    loop {
        if 27 <= round { break; }

        let copy_state = transformed_state;
        for i in 0..STATE_LENGTH {
            let aa = INDICES[i];
            let bb = INDICES[i + 1];
            let cc = (copy_state[aa] + (copy_state[bb] << 2) + 5) as usize;
            transformed_state[i] = TRUTH_TABLE[cc];
        }
        round += 1;
    }

    transformed_state
}

fn absorb(trits: [i8; TRITS_LENGTH], size: usize) -> [i8; STATE_LENGTH] {

    let mut state: [i8; STATE_LENGTH] = [0; STATE_LENGTH];
    let mut offset = 0;
    loop {
        if offset >= size { break; }
        
        let mut len = HASH_LENGTH;
        if size < HASH_LENGTH {
            len = size;
        }
        
        for count in 0..len {
            state[count] = trits[count + offset]
        }

        let transformed_state = transform(state);
        state = transformed_state;

        offset = offset + len;
    }

    state
}


fn hash2tryte(hash: [i8; STATE_LENGTH]) -> [u8; TRYTE_LENGTH]{
    let mut tryte: [u8; TRYTE_LENGTH] = [0; TRYTE_LENGTH];
    for count in 0..TRYTE_LENGTH {
        let mut n = hash[count * 3] + hash[count * 3 + 1] * 3 + hash[count * 3 + 2] * 9;
        if n < 0 {
            n = n + 27;
        }

        if n > 27 || n < 0 {
            panic!("illegal hash. {}", n);
        }

        if n == 0 {
            tryte[count] = CHAR_9_U8;
        } else {
            tryte[count] = n as u8 + CHAR_A_U8 - 1;
        }
    }

    tryte
}

#[cfg(test)]
mod tests {
    use super::*;

    const SYMBOLS_TRITS_EXPECTED: &str = "MPMZZPPZMMMPMZZPZPZPMZPMMPPPZMMPMPZMZMZZZZMPPPPMMZMPMPZMZPZMPMZPMZZPPMMMPPZZZPMPZPZPPPZPPMZZMMZZPZMZMMZMPMMPMZPMMZPZPPZMPPZMZZZPPMMMZPMZPMMMMMPZPPZZPPMZPZPMZMMPPZMMPPMPZMPMZZMZMMPMMPPZMMMMPZMZZPMPPPPPPMZMMPMZMMZZMMMPZPMMMZMZMZPZPZMPMPPMPMMZZZMMPPPZZZPPMZMPPZPPZMZZZPZMZMMMMZZZZPPMPZZZPMMPPPMMPPMPZZMPPPZZPZMMZMPZZZPPPZMMZPPZZZZPPZPMPPPMZMMPZPPPMZPZMMMZPMMMMMMZPPZPPZMZZZMPZPZZZPZMMZZPZPZMZZMZZZMPZZZMPPZZPPZZPMZPPZZPZPZMMPPPZPMMZMMPZZZZZZZPMPMMPMMZPPZPMMZMPMMZPMPZMMMZMPMPPZMPPMPMPZMPZMMPMPZPPZZMZMMMMPMZPZZMPMZPMMPMZMZMZZPMMPZZZZZZPZZPZPZPZZMZZMMMMZZPZZMMMZZMZMPMPZZZZMZZZMMPZPZPPMPMZZMZZPZZZZPZZZMMMZPZMZMZPPMZPPMZMMPZMZZZZPPMMZPPZZPMZMMMZPZZZPZMPPZZPMPZZMZMMPZMMPPMZMPMPPPPZPZZPPPPZPMMZPPZPMPMPMPZPMPZMMZMMPMZMPPMMZZMZPPZPPZMZZZPPPZMPPZMMPPMMZZPPPZMMMMMPZPMZPZMZPMMZPZPPZZZZZMPPMZZPPMMZZZZPMMMMMZZZZPPZPPPMMMZMMPZPZPZZMPZMMMPMPPPPMMPPMPPPZPMPZZZZPZPZZMMPMMZZZPZZMZPZMPPZMPZPPPPZPPMZPZZMPPPPZZMZZZZZZZMMPPMPMMMZPPZMPMZZPMMMPPZZPPZZPPZPZPMZMMMPZZPPPPZPZMPPZZMZMPMZMMPPZZPMZPPMPZPZPZPPZZMZPPMMZPPZZZPZPZMPPZPZMPZZZPMMMZPZZMMPPPMMZPZMZPMPZPMMMMMZZZZPPPMZMZMPZMZPZZZZMPMZMZZMMZMZPZZZZPZZZMMZMPZPPZZZPMPZMMMPMPZMPPMMZPPMZMPZMZZZMPZMZPMPMMMPPPMZZMZMMPPPZPZPZZZMZZMZPZZZMZPMMPPMMPZPZZPMMZZZZPMZZMPZPZMZZZPPPMMZZZMZZZPMMZMMZZPZPPPPPMPPZZMZPMPPZZZMPMZZZMMZZPZZMMPMPZPZZPPPPMMMZZPMZZMMPPMMPMPZMPPPPMMZZMZMMZZMMMMZZZMPPZZPMPPPMZPZMPZZMMPPZPPZZZZPPPPMZZZPPMPZPMZMZZPZZMMZZPMMPPMPZPPZMPZMMPPMZPZMZMZPMZPPPMMZZZPPZZMMZZPMZMZPPMPMMZMZMPZZPPMZPPZZZMPZPPPMMZMPPMPPZMPPMPMZZMZMMMPPMZPZPMMMZPZPMPZZMZMMPMZPMZPPZZZPPPZZPMZPPMZMPPMPZMZMZPMPPPMZPPMPMMPMMZZZZMZPPPMMPZMMPPPMMPPMZZPZPMZZMPZPZZZZPZPZPMPZMPMMMMPZZPMZZPZPMMMZPMZMMPMMMZMPMMPMMPPMZPZMZPMMPZZMPZMPMPMZPMZZPZZPZPPMPZZMZZPZZMPMMMPPPMMMPPPZZZZPMMMPZPZPMZMMPMMMMZMMMZMMMPMZMMZMMPZMPMPZMPZZMPMMZZPZPMPMPPMZPMMPMMZZPMMZPMMMZPPZPMZMPPZZMZMMMPZZZMZMMZPMPPMPZMPMMPPPPPZZPPZPMPPPMMPMZZMMZPPPPMMMMPPZMMMPPPPMZMPMMPZMPPMMPMPPZMZZMPPPZZMPMZPMMPMZMMPZZMMPPPMMMZZPZMMZZZZMZPZZPMPPMPPMZPPPPZMMMZMPMZPZPZMMZPMPMZZPPMPMZMPPMMMMPZZMPZMMPPMPZPPMMMZPZZZZMMZPPMPPZPMMMMPPZPMMMZPMPZMPMZZPMZZMPZZZMMZPZMMPMMMPMZPMMPMZMMMZZPZPZZZZZPPZMPMZMMMPMZMMMPMMMMPPZZMPPMPPZZMMPPZZMPZMPPMPPMPZMPPMMPZMPZPPPPZMMPZPPMMPZMMPMZPMMMZZMPMZPZPPMMPPZMMMPZZZPMMZPZZZZZMZPMMZMPMZPZMZMMPMZZZZZPPMPPMZPPMZMPPZZZMPPZZZMMZPPZMMPZPPPPZZMPPZMZZMPPZPZMPZPMZZMMZPZPMMPMPMMPPMPPMPMZZZPPZZZZPZPMPZMPZPMZZMPPPZZZZMPPZPMPZZPPPMPZPPMZZPZPZZPZPPMMMPZMZMPZMMMPZZMZZPPMZMZMMMZZMMZPPMZPPPZZMZZZPMZMPZPPPMPMZMMMMZPZPZZPPMPZPMPMMPZPMPPPMZPPMZZMMPMPMMMPMPMMMZPMPZMZZPPPMMPZMPMMMMZPPPPPPPZZMMPPMPZMMZZMPZMPPZPMPPMMZPMPZMZMPZZMMZMZZZPZZPPMMMZMMZMZZZPZZMPMMMMPPPZMPZZMZPZZPMPPPZPZMMPPPMMPMMZZPZZPZZMPPMPPPMPMZPZMPPZPMZMMMZZPZMZPMZPMZMPMMPPPZPMPMZMMMPPMZZZPMPMPZPPMPPMMMMMPZMZMZPMMZMZMZZZMZPPPMZMPPPMPZPZPMPMPPZPPZPMMPPPZMZMPZZZPZMPMMMZPMMPZMZMPZMMZMMMMMMPMMMMZPZPZPZZMPPZMZMMPZPZMZPZPMPZMZPPMMPPMMZMZZZZPPMPPPZMPPZZZZZMPMPMZPMPPPZZMZPPZMMPMZMMPZPPPZPZPMMPMZPZZPPZZZMMZMPZZZPPPZMPZPPPZZMZZPZPZMZZPPZPMPMPMZMZPMPZZZPMPMPMPPMZMPMPMZMZZPZMZZMZMMMZPPPZZPMMZZMMZMMMMMPMMMMPPMPPZPMPZMZPPMMZZMPZPZMMMPZMPZMZPPMZZMZZMZZZMZZMZMZZPMZZMMPMMZPMMZMZMZZZMPPPPZMZZPPPZPZMPMPMZMPMPPMMPPMMPMMPZZZMPPZPMPMPPPMPZZPPMPZMMZPMZPZZMMZPZPMMPZZPMZPZPMMMMPZZMPMZPPZZZMMPMZPZZPPZPMZMMPMZPZZPZPPPMZMZMPMZPZZPZPPPMPMZPPMPMMPPPMZMZMPPMZPPPZZPZMMMPPPPZZMZPPPZPMMZPMPMPMPZMZMPMZPMZMPZZMPZZZZPZZZMZZPMPPPPZPPMMZZMZZZZZMPPZZPZMPMMPZPPZPMZPMMPPPMMZMZPMMZZMZZPZMPPPMMPMMZZPMMZMZZZMMPZMPPPZPMPZZZMZPPMMPPMPZPMPZPMPPZMZMZZPMMPMZPMPZMZZZMMPPZMMPZZMZPZZMZMMZMPZMZMMPMMMPZZZPMPZZPMZZMMPZMMZMMPZPZPPMPPPPPPPMZPZZPPPZZPMMMMPZMPMZPMZPMPMZMPZMPPZPZPPPZMPPPPPPMMMMMMPZPMMMPMZPMMMZPPPZZPPMZMZZZPZMMZPZZMPZPMZZPPPPZPPPMPMZMZPZZPMPMMZZMPPMZMMPZMZZZMPMPZPPZMMPMZPPMPMMMMMPZMZMMPPMPMZZZPMPMMMPPMMZPZZMZPPPMPMPPPPMMPPMZPPMPZMZMPMMMPZMMMMZMZPMMPZZZPMPMMMPPPPZZZZZZMPPPPMZZMMPMPMMMMMPZZPZZMMZPMMZZMZZZZZMMZMPMZMPZZPPZMPMZZPPZPPMMMZPMZPPZPZPZMZMZMZZMPMPMPMPPMPZZZMPPPMMMPZZZZZZMPPMMPZZZPMZPZMZPZMMPMPMMPZZZPMMPZPPZZPMMZPMMMPPMZZPMZZMPPZZZZMPZMPMMPPMZPPMPMPPZMPPMMPPZZMPPZPPZPZMPPZPMZPPMZZMZPZPPMPPZMMMZMMZZPPPPPMZPPZMPZPPZMPZZZMZMMZPMPMZZZZPPZPPPZMZZZMZMMZMZMMPPZPZMZMPPMZZPMZZPPMMMPPPZMMPZPPMPZMMPPPZZPZPPPPPPMMMZZZZMZMZPMZZPZPPMZMPZMMPPMZPMZZPPPPZZZMZPMPPPMMZPZPMPZPMPZPPPMPMPZZMPMPMMZZPPPZMZPPZPPPMZPMMPZMZZMMZZPMZMMPMZMMMZMMZMMPPPMMPPZPPPMZMMZPMZMZMZPPZZZPZZPMMZPPPZPPMMPPMZZZZZMZZPPZMPMPZMZZPMZPMZMZMMMPMZMMPPMZPMPPPMMPZZPPZMPPPPZZZZZPPMMZZZZPZMPMZMMPMPMMMPZZMZZMZPPZMZPPPZZPPPPMZPPMPPMZZMZZMZPZPPPMZZPPPMMZZPMMPZZZPZMMPPZMMMMPPZPZMZPZZZMPPPZMPPZZPZPZZMMPMZZZPZMMPPMPPMPZZMMPZMZZPZZZZPPMMZZPZMZMPPZMPPZMPMPZMMZPPPMMMPZMMPZMMZPZZPMPMZZZPZPMZPPPPMPZMPZPMMMZMMPMZZMPMPZMMZZMMMPZZPPMZMZMPZMMPZZPZPPPMZPPZPMZMZZPZMZMPMMPMMZMMMZZMMZPZMPPPPPZPZPZMPMPMPZZZMMPMPZPZZMZZMPMZMMZPMMZMZPZZPMPMMPZPPZZPZMPPMMPMPZZZPMZZMZZZZZMPMMMMMMPMZPMPPZMPZPZMZPZPPMZMPMMZPZZPPPPMMZPZMZMZMZZMZPZZZPZPMZMZZPPMZZMZMPPZMPZMZMMPZMMMZPZMMMMPZZZPMZMMMMZPPZPZZPPPZPZZPZPZPMPMPPMMMMPMMMMZZMZPZMMZZPZMPMMPMZPMPZPZMZPZMZMZZPMPMPPMZMPMZPZPMZMMMMPPZMPPMZMZZMMPZPPMMZPZMMMMPMZPMMZZZZMZZZZPPPPMMZMZPZZMZPMZZPMMMZZZZPPZZZPMPMMMPZMZPMMPMPZZZMPPMMZPMZPPPMPZMZZMZMMZMZPZMMPPZMMMMZMPPPMZPMZMMMPZMZMPMPZPZZZMZZMMPPZMPMZPZMMMMPMPPMPMMZPPPZMZPZPMMMMZZMZMZPZZPPZZMZZPMPPPZPZMMZZMPPZPPPPMZZMPMPPPMZPMMMZZZPPMMMZZPPMZZPZZMPMMPZZZZZMPZMPZZZMZPMPZPMPZPPZMPZZPMPMMMMMMMPPMZMPZPZMZMZZPZPZPMMZZMPZMPMPPPMZMPPZZPZZMPMMMZPMZPZPMPMZZZPMPZPMPZPZZPPZZMZPMMMMMMPZPMPZMMPZPZPMZPZMMMZPZMPMPMMPPZPPMZZPPMZPPZPPMMZPMZMMPMZMMPPZMZPPPPMZZPMMPZPPMPZPPPPZMZPPZZMMZZPZPZPPPMMMPPPPZMPPZMPPMPPZZPZPPZMPZMMMZMZZZPZMZMPPZZPPZMPPPZMMPZMZMMZPZZPMMZZPZPPMZPZPPMMMMZPMPPZPZZZMPPPPMMZPPMZMZPMPMZMMPPZMPZZMMPZZPMMZMMZMMMMPZZPZMMPZZPPMZPZMZMMMZPMPPPMMZZPZPPMPMZMZPMMPZZZMPZZPMZZZZZPZPPPPMPPZMPMPPPZPMZPMPMZPPMZPZZMMZPPMPMZZMZMZPPZPZMMPPPMPZPPZPPPPPZPMZPMMZMPMZPMMPPPPPMPPZMMMMZMPMMZZMMZPMPPPMMZZPMPMZMPZZZMMPZZMPPPPMPMPZMZMZZMMPMZZMPZPPPPMZPMMPPMMMZZPMPMPPMZPZPMPZZZPZMPZMZMPZPZZZPZMZZZPPPZZPPZZMPZMPMPZMZZZZPZMZMPPMMZPZMMZZZPPPPZZMMZPZPMPZZMMZMPZZMZZPPZZPZZMPPMMZZMPZZMZZPMZZMZZPMPPPMMMZZMPZPZZZZPMMPPZPMMPPZZMPZZPPMPZMPPMPPPZPZMMPMMMPPZPZPPZZMPZMZZZZPMPMPPZMZMZPPMMMZPZZMZMZZMMMZMZZPZZMMMZPMPPZPPPZZMPMZMPZMPZMZMPPPMMZZZZMPZMPPPZPMZPZMPZZZMZZZPPZMZZPPZMPPPPPMPMZPMMZMZPMMMMZPMZPMMPPMMMPPZZPMZZZMMMMPZZZZPMPMMMZPZPPZMMPMPMPZMZZZMPZZPPPPPPMZZZZPMZZZMPZZZZZPZPMZMMZZMZZMMMPMMZPZMZMPZPMMMMMZZZMMZMZPPZPPMZPZPPPMPZMZZPZPPZZMMPMPMZZZPMMPPMMZPZPZMZMMPMPPPPPZMPMPPMPPMZMZZPPZMZPPZZZPZMZZMPPZMMZPPMMZMZMMZMPPMPZMZZMPZPPZPZPPZZPZZZMZPZMPPZMMPZMPZMZPMMPZZZZMMPZZPMZZZPMPPZPPZPPZZMPZPMMPPPZPZPPZZZZZPPPMPMZMPZPZPPZZMZZPPMZMPPPMPMZMZMMZZMPMPZPZPPZPPZPZZMPZPZZZZPPZZMZMZPZZZZPPPMZZPPZMZMZPPZMZZMPPZMPZZZMMZPPMMMPMZPPPMMZZPZPZPZMMPPZMMPMMPZPMPZMZZPMZPMMPPPMMZPMZMPMMPZZZZPZMPMZMZPPZZZMMPZPZMPZMPZPPMMZZPMMMZPPMZZPMZZZMZPPPZMPZZMMPMZZMZMPMMPPMMZZMMZZPZZMPMPMZPZPMPZPPPPPPMPPMZZZZPMMPPMZPMZPZZMMMPPPZZZPZPMZZZZPZZZMMZZZMPZPMMZZMPPPZZPPPZPMPPMZZMZZPMMPPPPMPPMMPMPPZZPPPPPZZPMPZMZPZMPPPPMPMPMMMPPZPMMZZPZMMZMMZPZZPPPZZMPMMMMZMMMZZPMMMMPMMMPMPMZMMMPMZZMPPPZMMPPZMPMZMMZZMPPMMMPZPMZZMZZMPMZZZZZZMMPMPPZPMZMZMZZPMMPMMZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZPZPPPZZMMPPMZPMMZPPZZZZZZZPZZZZZZZZZZZZZZZZZZZZZZZZZZZPZZZZZZZZZZZZZZZZZZZZZZZZZMPMMMMZMPPZZMPPPPMMPZZMPMZPMZPPPPMPPMZMMPPMPPPZMMMMPZPZZMZPPPPZZZPPMZZPMZZPPMPZMMPPPMMPZMMPZZPPZPZPZPPMMMPZMZZZPZPPMPMPPPMPZPMZZPMZPMZZMZPZZPZPMPPZMMMMZMPPPPPMPMZZPZPZZZZZPZMMZZZPZZPPMZMZMMMMPPPMPZPPMMZPMPMPMZMPMPMMZMPMPPPMMPMPMZPPPMPPPPZPMMZMPPPPZZMPPMZPMZPMZPMZPMPMMPZMZPPPMMMZPMZPZMPZPPZMZMZZZMPPMMPZZZMPPPMZPPPMPPMZZZMMZZPMMPMMMMPPPMMMZMPPPMPPZMMZZPPMZPMZMPMPZMPMMPZPPMPMMPPMZZZMMZZPPZMZZMMMPPMMZPMPZPZMZMMMZZMMPZZZPPZZMZMPPMMZMMMZZPZZPZMPPMPZZZZPPZMMPMZZMMPZMZPZMZZZZZZZZZZZZZZZZZZPPPZZZMMPMZZPMPZZMZMZPZPZZZMMZPMZPZMMZZZMZMMMPPZMZZMPZPZZPZPMZZPPPPZPZZZPPMPZZPPPZPMMMZMMZMMZPZZMMMPMPZZPPPMZMPMPPZPZZPMMMMZZMMPZZZZPMPZPPMZZMZZMPPZZMZZPPPPMMPZMPZPMMPPZZZMPPMMPZMPMZZZMZPMZPPZPMPMPZMZMPZPZZPZPMZZZPZMMZMZPMPPPZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ";
    const SYMBOLS_STATE_EXPECTED: &str = "PPPMZMMMPZMZMMMZPMZPMZPMMMZPMPPZZMMMMMZMPZPZZPZPZZMZMMPZMMZMPMPMZZPPPMZZMZZZMPPMZPMPPMMMMPPPMPPPZPMPMMPPZZPZZZZPPZMPZPZPMZPPPPMMMMPZPMPZPZMZPZPZMMMPZMPMPMZZZPPMPZZPZMPMPPMZPMPMZMPZZZZMMZPZZZZMZZPPMZZMZZPPMPMMMZPPMPMZMZPPZPZPPPZMMZZMZPPMMPMMPZPMZMMMZZMZMPPZZMZMPMMZZPZZZZPZPPMPZMPMZMPPPZZZPPZMPPPPZZPZMZPMMZPPPZMMPMZPZZMMMPZZZMZPMPMZMZZMPPPZZZZMMZMMMMZPZPZZZZMMZPMPZPPMMZPMPMPPMZPPPPMPPPPPMZMMZZZZZMPMZMZPPMMMZMZMMMZPMPZPMZZPZZZMMPMPZZPZZZZMPPMMPMPZMPPPPMPMPZZPPPMPMPPZPZZMPMZMZZZZZZZPPPMZZPPZMPZZZPPPPPPMZZPPMMZPMMMZZMPZMPPPZMPMPMPPMMZZZPZZZZMMPZZZMZPMPZPPZMZMZMZZPZZZMZMZMPMZPPZPPMPMPZPPMPPZPMMPZPZZZZMPPZMMZZPMPPZPMPZZMMZMZZMMMZZPZPMMZMPMZMZMMPZZMZPZMMZMPMPZMMMMMZPMMMMMMZZZPPZPZPPZPPPMZPZPMMMPZMMZZPZPZPPZZPPZPPMPMZMMPPZPZPZPM";

    const HASH_EXPECTED: &str = "MQEXNUUUWGANWBAJROSQGZMZZFYGPEVMUPDC9DBJHMNBGCHCNSGZLBCTVUYB9WARIYXLTWVYHJLSZHPPJ";
    
    #[test]
    fn test_tx2trit_to_absorb() {

        assert_eq!(TX_LENGTH, TX_STRING.len());

        // tx2trit
        // TX(string TX_LENGTH:2673) -- string to [u8] --> TRITS(-1/0/1 TRITS_LENGTH:TX_LENGTH*3)
        let trits = tx2trit(TX_STRING);
        let symbols_trits = trits2symbol_tryte(trits);

        assert_eq!(SYMBOLS_TRITS_EXPECTED, symbols_trits);
        assert_eq!(TRITS_LENGTH, symbols_trits.len());

        // absorb(and transform)
        // TRITS -- [u8] to [u8] --> STATE(-1/0/1 STATE_LENGTH)
        let state = absorb(trits, TRITS_LENGTH);
        let symbols_state = trits2symbol_state(state);

        assert_eq!(SYMBOLS_STATE_EXPECTED, symbols_state);
        assert_eq!(STATE_LENGTH, symbols_state.len());

        // STATE_LENGTH : 3 * HASH_LENGTH

        // absorb(and transform)
        // STATE -- [u8] to String --> TRYTE(9+A-Z HASH_LENGTH / 3)
        let tryte = hash2tryte(state);
        let symbols_tryte = tryte2symbol(tryte);

        assert_eq!(HASH_EXPECTED, symbols_tryte);
        assert_eq!(TRYTE_LENGTH, tryte.len());
    }

    fn tryte2symbol(tryte: [u8; TRYTE_LENGTH]) -> String {
        let converted: String = String::from_utf8(tryte.to_vec()).unwrap();
    
        converted
    }

    fn trits2symbol_tryte(trits: [i8; TRITS_LENGTH]) -> String
    {
        let mut trits_symbol: [char; TRITS_LENGTH] = ['0'; TRITS_LENGTH];
        for count in 0..TRITS_LENGTH {
            trits_symbol[count] = trite2char(trits[count]);
        }

        trits_symbol.iter().collect()
    }

    fn trits2symbol_state(trits: [i8; STATE_LENGTH]) -> String
    {
        let mut trits_symbol: [char; STATE_LENGTH] = ['0'; STATE_LENGTH];
        for count in 0..STATE_LENGTH {
            trits_symbol[count] = trite2char(trits[count]);
        }

        trits_symbol.iter().collect()
    }

    fn trite2char(trite: i8) -> char {
        
        let trite_symbol;
        if trite == -1 {
            trite_symbol = 'M';
        } else if trite == 1 {
            trite_symbol = 'P';
        } else if trite == 0 {
            trite_symbol = 'Z';
        } else {
            panic!("unknown trits {}", trite);
        } 

        trite_symbol
    }
}