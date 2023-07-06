use std::collections::HashMap;
pub fn return_toio_mac(id: usize) -> String {
    const MACARR: [&str; 193] = [
        "Mac Address",                //0
        "ee:df:8d:65:9e:ca",          //1
        "f5:2f:79:72:9f:76",          //2
        "f7:2f:f7:48:67:bd",          //3
        "ec:5c:12:4c:8a:a6",          //4
        "f5:eb:4e:40:64:6a",          //5
        "c8:d2:86:b9:b1:a4",          //6
        "e3:d7:a8:cc:7f:8b",          //7
        "de:bf:4e:ee:fb:1e",          //8
        "d2:28:a1:41:cc:d3",          //9
        "dd:7e:90:3e:ec:b1",          //10
        "d4:1a:22:ee:90:57",          //11
        "f9:26:84:9b:e1:37",          //12
        "c4:0a:26:36:12:3a",          //13
        "ed:53:f1:e8:1f:6f",          //14
        "e7:54:2f:68:41:d9",          //15
        "df:28:e5:e8:16:ec",          //16
        "d0:6f:bc:af:b3:71",          //17
        "ee:6a:c3:bc:36:8e",          //18
        "cf:18:75:4b:76:46",          //19
        "fe:9b:3c:cd:40:1c",          //20
        "da:25:61:d4:1d:f7",          //21
        "ff:de:4d:9d:70:a9",          //22
        "f7:c3:77:83:89:25",          //23
        "fd:ef:a0:b0:5f:09",          //24
        "f:9d:b1:d3:63:d0",           //25
        "fc:7e:6f:56:e5:b3",          //26
        "c6:64:fd:be:7a:34",          //27
        "dc:60:80:fb:52:b9",          //28
        "dd:40:2e:84:0f:50",          //29
        "f7:57:cf:e6:f0:2c",          //30
        "fe:5a:84:26:74:35",          //31
        "e7:02:66:1f:80:0f",          //32
        "ea:45:bd:06:b2:af",          //33
        "f6:05:ba:47:ef:3e",          //34
        "d3:9e:ee:9b:4e:53",          //35
        "fe:50:b0:28:5f:6b",          //36
        "ee:91:3e:98:24:59",          //37
        "cd:ab:b3:7c:a8:6d",          //38
        "fe:80:3e:da:1d:a6",          //39
        "c9:81:9c:fe:93:2b",          //40
        "ce:37:16:3d:9d:61",          //41
        "f0:bf:cd:74:88:f6",          //42
        "c3:7f:85:0d:69:4d",          //43
        "ee:74:a0:9b:b4:6c",          //44
        "d0:8e:44:8d:ce:0c",          //45
        "fe:de:04:86:a5:8e",          //46
        "e2:b3:78:08:c6:2f",          //47
        "e3:aa:d2:f6:9d:97",          //48
        "dc:71:4c:d2:f0:f9",          //49
        "ee:35:b2:5d:56:50",          //50
        "c5:3a:90:1f:a6:cf",          //51
        "d8:7a:ee:b6:b0:14",          //52
        "f9:4b:a1:54:5f:50",          //53
        "e9:5d:07:ad:72:4e",          //54
        "d4:6b:59:ad:12:71",          //55
        "f0:9d:71:e9:08:ad",          //56
        "fb:34:2a:6f:24:31",          //57
        "e5:f3:76:7a:58:ed",          //58
        "ed:97:12:49:99:75",          //59
        "ec:11:dc:22:70:2d",          //60
        "e5:b8:e0:f1:82:56",          //61
        "f5:f9:ce:05:fc:7c",          //62
        "e8:05:d5:97:f8:a1",          //63
        "e5:24:46:76:b4:04",          //64
        "ff:ba:e1:66:24:44",          //65
        "e7:46:a5:2a:8f:5c",          //66
        "f1:dd:37:f8:f2:a4",          //67
        "da:96:a8:7c:30:8f",          //68
        "cf:e5:04:97:46:d8",          //69
        "df:43:09:2d:63:10",          //70
        "place",                      //71
        "place",                      //72
        "place",                      //73
        "place",                      //74
        "place",                      //75
        "place",                      //76
        "place",                      //77
        "place",                      //78
        "Not needed for Alex's code", //79
        "ef:54:01:c5:69:fb",          //80
        "Not needed for Alex's code", //81
        "Not needed for Alex's code", //82
        "Not needed for Alex's code", //83
        "Not needed for Alex's code", //84
        "c2:6b:a4:ab:a0:07",          //85
        "Not needed for Alex's code", //86
        "Not needed for Alex's code", //87
        "Not needed for Alex's code", //88
        "e2:f3:31:c8:5b:1d",          //89
        "Not needed for Alex's code", //90
        "Not needed for Alex's code", //91
        "ff:6d:30:1b:a6:42",          //92
        "Not needed for Alex's code", //93
        "fa:7f:02:d1:67:1e",          //94
        "e0:b7:69:dd:05:84",          //95
        "Not needed for Alex's code", //96
        "Not needed for Alex's code", //97
        "Not needed for Alex's code", //98
        "fc:1a:4f:90:01:ca",          //99
        "f8:95:ed:8e:a8:90",          //100
        "ff:71:f1:7d:84:a4",          //101
        "Not needed for Alex's code", //102
        "d1:f4:60:8a:35:b8",          //103
        "Not needed for Alex's code", //104
        "Not needed for Alex's code", //105
        "c1:5f:2a:b8:40:0d",          //106
        "dc:68:a2:44:ea:37",          //107
        "db:8e:a5:8c:34:71",          //108
        "d3:f8:98:a3:88:a9",          //109
        "d9:bd:5f:01:fd:b5",          //110
        "da:19:7a:4d:f3:1e",          //111
        "Not needed for Alex's code", //112
        "d2:4f:e5:3a:0d:0a",          //113
        "c9:70:e7:b1:95:b7",          //114
        "Not needed for Alex's code", //115
        "d6:83:11:ab:8d:97",          //116
        "ca:f6:9a:de:a7:90",          //117
        "e7:c3:90:16:9e:db",          //118
        "ce:02:2c:68:be:68",          //119
        "Not needed for Alex's code", //120
        "cd:17:c1:82:31:4c",          //121
        "e0:fd:27:32:7e:b8",          //122
        "Not needed for Alex's code", //123
        "cf:04:c8:02:d0:7a",          //124
        "Not needed for Alex's code", //125
        "Not needed for Alex's code", //126
        "ed:78:af:de:81:0f",          //127
        "e0:0e:e1:7b:73:7a",          //128
        "fd:cc:98:30:7e:59",          //129
        "cb:13:33:11:10:52",          //130
        "place",                      //131
        "ec:08:67:4b:3e:ea",          //132
        "d0:0a:fd:a1:95:2c",          //133
        "cb:0c:56:e1:ab:44",          //134
        "ce:4c:fa:83:b5:ad",          //135
        "f7:bb:b8:14:fd:9b",          //136
        "e7:36:a3:23:46:47",          //137
        "da:ea:9c:51:31:0f",          //138
        "e2:52:72:bb:fe:8a",          //139
        "e9:fa:9e:3f:33:05",          //140
        "e5:51:4f:95:7e:92",          //141
        "f5:ee:0c:e7:16:ba",          //142
        "ce:49:49:10:dc:0b",          //143
        "e8:a1:ef:8e:0c:b3",          //144
        "db:7c:5c:7b:b5:fa",          //145
        "cc:a0:b0:69:20:e3",          //146
        "e6:f3:37:33:84:8d",          //147
        "e3:10:7b:f5:86:03",          //148
        "dc:76:c4:4c:56:47",          //149
        "d3:fe:ee:7a:e2:dd",          //150
        "f4:57:66:e3:fd:9a",          //151
        "c3:45:c7:a1:70:0d",          //152
        "f2:25:56:79:2c:b2",          //153
        "c9:43:41:f6:5e:d9",          //154
        "fb:7e:30:80:a8:aa",          //155
        "c1:a5:f7:63:65:ad",          //156
        "f9:95:12:c0:fa:46",          //157
        "e4:d5:72:4b:9d:36",          //158
        "de:7b:35:a4:d8:a6",          //159
        "c8:86:3b:19:df:b4",          //160
        "db:87:cf:92:15:cb",          //161
        "f2:16:95:09:91:f4",          //162
        "cb:2d:90:38:67:05",          //163
        "c7:37:40:a2:1b:22",          //164
        "c9:2f:02:32:8b:65",          //165
        "df:67:ef:ec:4a:44",          //166
        "f7:85:35:e1:e7:77",          //167
        "e2:6c:c7:32:64:97",          //168
        "e5:38:91:2a:fc:c8",          //169
        "Not yet found",              //170
        "d5:18:af:9e:e0:11",          //171
        "d5:a3:3d:5c:bf:87",          //172
        "f3:c9:18:47:34:c7",          //173
        "c8:06:1f:db:b9:fc",          //174
        "ca:81:d3:36:83:3f",          //175
        "fd:54:56:89:21:57",          //176
        "ee:09:36:20:91:5e",          //177
        "d1:21:24:e1:09:7c",          //178
        "d6:2d:ba:39:2c:56",          //179
        "f2:f6:84:f8:66:ac",          //180
        "d5:dc:b8:5a:9e:c4",          //181
        "e3:29:98:4e:75:d4",          //182
        "ea:2c:39:d9:62:99",          //183
        "e7:35:2f:7e:65:09",          //184
        "c2:09:3f:7f:c5:90",          //185
        "d6:6c:4e:59:60:5a",          //186
        "db:cd:4f:72:e4:ac",          //187
        "f9:95:c7:65:fa:57",          //188
        "f1:1d:cd:6a:88:a4",          //189
        "c6:b1:3d:58:4b:c3",          //190
        "d5:d6:9a:ef:a0:09",          //191
        "df:ad:5c:5e:7b:c1",          //192
    ];
    return MACARR[id].to_string();
}

pub fn return_toio_id(mac: &str) -> i32 {
    let idmap: HashMap<String, i32> = HashMap::from([
        ("ee:df:8d:65:9e:ca".to_string(), 1),
        ("f5:2f:79:72:9f:76".to_string(), 2),
        ("f7:2f:f7:48:67:bd".to_string(), 3),
        ("ec:5c:12:4c:8a:a6".to_string(), 4),
        ("f5:eb:4e:40:64:6a".to_string(), 5),
        ("c8:d2:86:b9:b1:a4".to_string(), 6),
        ("e3:d7:a8:cc:7f:8b".to_string(), 7),
        ("de:bf:4e:ee:fb:1e".to_string(), 8),
        ("d2:28:a1:41:cc:d3".to_string(), 9),
        ("dd:7e:90:3e:ec:b1".to_string(), 10),
        ("d4:1a:22:ee:90:57".to_string(), 11),
        ("f9:26:84:9b:e1:37".to_string(), 12),
        ("c4:0a:26:36:12:3a".to_string(), 13),
        ("ed:53:f1:e8:1f:6f".to_string(), 14),
        ("e7:54:2f:68:41:d9".to_string(), 15),
        ("df:28:e5:e8:16:ec".to_string(), 16),
        ("d0:6f:bc:af:b3:71".to_string(), 17),
        ("ee:6a:c3:bc:36:8e".to_string(), 18),
        ("cf:18:75:4b:76:46".to_string(), 19),
        ("fe:9b:3c:cd:40:1c".to_string(), 20),
        ("da:25:61:d4:1d:f7".to_string(), 21),
        ("ff:de:4d:9d:70:a9".to_string(), 22),
        ("f7:c3:77:83:89:25".to_string(), 23),
        ("fd:ef:a0:b0:5f:09".to_string(), 24),
        ("f:9d:b1:d3:63:d0".to_string(), 25),
        ("fc:7e:6f:56:e5:b3".to_string(), 26),
        ("c6:64:fd:be:7a:34".to_string(), 27),
        ("dc:60:80:fb:52:b9".to_string(), 28),
        ("dd:40:2e:84:0f:50".to_string(), 29),
        ("f7:57:cf:e6:f0:2c".to_string(), 30),
        ("fe:5a:84:26:74:35".to_string(), 31),
        ("e7:02:66:1f:80:0f".to_string(), 32),
        ("ea:45:bd:06:b2:af".to_string(), 33),
        ("f6:05:ba:47:ef:3e".to_string(), 34),
        ("d3:9e:ee:9b:4e:53".to_string(), 35),
        ("fe:50:b0:28:5f:6b".to_string(), 36),
        ("ee:91:3e:98:24:59".to_string(), 37),
        ("cd:ab:b3:7c:a8:6d".to_string(), 38),
        ("fe:80:3e:da:1d:a6".to_string(), 39),
        ("c9:81:9c:fe:93:2b".to_string(), 40),
        ("ce:37:16:3d:9d:61".to_string(), 41),
        ("f0:bf:cd:74:88:f6".to_string(), 42),
        ("c3:7f:85:0d:69:4d".to_string(), 43),
        ("ee:74:a0:9b:b4:6c".to_string(), 44),
        ("d0:8e:44:8d:ce:0c".to_string(), 45),
        ("fe:de:04:86:a5:8e".to_string(), 46),
        ("e2:b3:78:08:c6:2f".to_string(), 47),
        ("e3:aa:d2:f6:9d:97".to_string(), 48),
        ("dc:71:4c:d2:f0:f9".to_string(), 49),
        ("ee:35:b2:5d:56:50".to_string(), 50),
        ("c5:3a:90:1f:a6:cf".to_string(), 51),
        ("d8:7a:ee:b6:b0:14".to_string(), 52),
        ("f9:4b:a1:54:5f:50".to_string(), 53),
        ("e9:5d:07:ad:72:4e".to_string(), 54),
        ("d4:6b:59:ad:12:71".to_string(), 55),
        ("f0:9d:71:e9:08:ad".to_string(), 56),
        ("fb:34:2a:6f:24:31".to_string(), 57),
        ("e5:f3:76:7a:58:ed".to_string(), 58),
        ("ed:97:12:49:99:75".to_string(), 59),
        ("ec:11:dc:22:70:2d".to_string(), 60),
        ("e5:b8:e0:f1:82:56".to_string(), 61),
        ("f5:f9:ce:05:fc:7c".to_string(), 62),
        ("e8:05:d5:97:f8:a1".to_string(), 63),
        ("e5:24:46:76:b4:04".to_string(), 64),
        ("ff:ba:e1:66:24:44".to_string(), 65),
        ("e7:46:a5:2a:8f:5c".to_string(), 66),
        ("f1:dd:37:f8:f2:a4".to_string(), 67),
        ("da:96:a8:7c:30:8f".to_string(), 68),
        ("cf:e5:04:97:46:d8".to_string(), 69),
        ("df:43:09:2d:63:10".to_string(), 70),
        ("place".to_string(), 71),
        ("place".to_string(), 72),
        ("place".to_string(), 73),
        ("place".to_string(), 74),
        ("place".to_string(), 75),
        ("place".to_string(), 76),
        ("place".to_string(), 77),
        ("place".to_string(), 78),
        ("Not needed for Alex's code".to_string(), 79),
        ("ef:54:01:c5:69:fb".to_string(), 80),
        ("Not needed for Alex's code".to_string(), 81),
        ("Not needed for Alex's code".to_string(), 82),
        ("Not needed for Alex's code".to_string(), 83),
        ("Not needed for Alex's code".to_string(), 84),
        ("c2:6b:a4:ab:a0:07".to_string(), 85),
        ("Not needed for Alex's code".to_string(), 86),
        ("Not needed for Alex's code".to_string(), 87),
        ("Not needed for Alex's code".to_string(), 88),
        ("e2:f3:31:c8:5b:1d".to_string(), 89),
        ("Not needed for Alex's code".to_string(), 90),
        ("Not needed for Alex's code".to_string(), 91),
        ("ff:6d:30:1b:a6:42".to_string(), 92),
        ("Not needed for Alex's code".to_string(), 93),
        ("fa:7f:02:d1:67:1e".to_string(), 94),
        ("e0:b7:69:dd:05:84".to_string(), 95),
        ("Not needed for Alex's code".to_string(), 96),
        ("Not needed for Alex's code".to_string(), 97),
        ("Not needed for Alex's code".to_string(), 98),
        ("fc:1a:4f:90:01:ca".to_string(), 99),
        ("f8:95:ed:8e:a8:90".to_string(), 100),
        ("ff:71:f1:7d:84:a4".to_string(), 101),
        ("Not needed for Alex's code".to_string(), 102),
        ("d1:f4:60:8a:35:b8".to_string(), 103),
        ("Not needed for Alex's code".to_string(), 104),
        ("Not needed for Alex's code".to_string(), 105),
        ("c1:5f:2a:b8:40:0d".to_string(), 106),
        ("dc:68:a2:44:ea:37".to_string(), 107),
        ("db:8e:a5:8c:34:71".to_string(), 108),
        ("d3:f8:98:a3:88:a9".to_string(), 109),
        ("d9:bd:5f:01:fd:b5".to_string(), 110),
        ("da:19:7a:4d:f3:1e".to_string(), 111),
        ("Not needed for Alex's code".to_string(), 112),
        ("d2:4f:e5:3a:0d:0a".to_string(), 113),
        ("c9:70:e7:b1:95:b7".to_string(), 114),
        ("Not needed for Alex's code".to_string(), 115),
        ("d6:83:11:ab:8d:97".to_string(), 116),
        ("ca:f6:9a:de:a7:90".to_string(), 117),
        ("e7:c3:90:16:9e:db".to_string(), 118),
        ("ce:02:2c:68:be:68".to_string(), 119),
        ("Not needed for Alex's code".to_string(), 120),
        ("cd:17:c1:82:31:4c".to_string(), 121),
        ("e0:fd:27:32:7e:b8".to_string(), 122),
        ("Not needed for Alex's code".to_string(), 123),
        ("cf:04:c8:02:d0:7a".to_string(), 124),
        ("Not needed for Alex's code".to_string(), 125),
        ("Not needed for Alex's code".to_string(), 126),
        ("ed:78:af:de:81:0f".to_string(), 127),
        ("e0:0e:e1:7b:73:7a".to_string(), 128),
        ("fd:cc:98:30:7e:59".to_string(), 129),
        ("cb:13:33:11:10:52".to_string(), 130),
        ("place".to_string(), 131),
        ("ec:08:67:4b:3e:ea".to_string(), 132),
        ("d0:0a:fd:a1:95:2c".to_string(), 133),
        ("cb:0c:56:e1:ab:44".to_string(), 134),
        ("ce:4c:fa:83:b5:ad".to_string(), 135),
        ("f7:bb:b8:14:fd:9b".to_string(), 136),
        ("e7:36:a3:23:46:47".to_string(), 137),
        ("da:ea:9c:51:31:0f".to_string(), 138),
        ("e2:52:72:bb:fe:8a".to_string(), 139),
        ("e9:fa:9e:3f:33:05".to_string(), 140),
        ("e5:51:4f:95:7e:92".to_string(), 141),
        ("f5:ee:0c:e7:16:ba".to_string(), 142),
        ("ce:49:49:10:dc:0b".to_string(), 143),
        ("e8:a1:ef:8e:0c:b3".to_string(), 144),
        ("db:7c:5c:7b:b5:fa".to_string(), 145),
        ("cc:a0:b0:69:20:e3".to_string(), 146),
        ("e6:f3:37:33:84:8d".to_string(), 147),
        ("e3:10:7b:f5:86:03".to_string(), 148),
        ("dc:76:c4:4c:56:47".to_string(), 149),
        ("d3:fe:ee:7a:e2:dd".to_string(), 150),
        ("f4:57:66:e3:fd:9a".to_string(), 151),
        ("c3:45:c7:a1:70:0d".to_string(), 152),
        ("f2:25:56:79:2c:b2".to_string(), 153),
        ("c9:43:41:f6:5e:d9".to_string(), 154),
        ("fb:7e:30:80:a8:aa".to_string(), 155),
        ("c1:a5:f7:63:65:ad".to_string(), 156),
        ("f9:95:12:c0:fa:46".to_string(), 157),
        ("e4:d5:72:4b:9d:36".to_string(), 158),
        ("de:7b:35:a4:d8:a6".to_string(), 159),
        ("c8:86:3b:19:df:b4".to_string(), 160),
        ("db:87:cf:92:15:cb".to_string(), 161),
        ("f2:16:95:09:91:f4".to_string(), 162),
        ("cb:2d:90:38:67:05".to_string(), 163),
        ("c7:37:40:a2:1b:22".to_string(), 164),
        ("c9:2f:02:32:8b:65".to_string(), 165),
        ("df:67:ef:ec:4a:44".to_string(), 166),
        ("f7:85:35:e1:e7:77".to_string(), 167),
        ("e2:6c:c7:32:64:97".to_string(), 168),
        ("e5:38:91:2a:fc:c8".to_string(), 169),
        ("Not yet found".to_string(), 170),
        ("d5:18:af:9e:e0:11".to_string(), 171),
        ("d5:a3:3d:5c:bf:87".to_string(), 172),
        ("f3:c9:18:47:34:c7".to_string(), 173),
        ("c8:06:1f:db:b9:fc".to_string(), 174),
        ("ca:81:d3:36:83:3f".to_string(), 175),
        ("fd:54:56:89:21:57".to_string(), 176),
        ("ee:09:36:20:91:5e".to_string(), 177),
        ("d1:21:24:e1:09:7c".to_string(), 178),
        ("d6:2d:ba:39:2c:56".to_string(), 179),
        ("f2:f6:84:f8:66:ac".to_string(), 180),
        ("d5:dc:b8:5a:9e:c4".to_string(), 181),
        ("e3:29:98:4e:75:d4".to_string(), 182),
        ("ea:2c:39:d9:62:99".to_string(), 183),
        ("e7:35:2f:7e:65:09".to_string(), 184),
        ("c2:09:3f:7f:c5:90".to_string(), 185),
        ("d6:6c:4e:59:60:5a".to_string(), 186),
        ("db:cd:4f:72:e4:ac".to_string(), 187),
        ("f9:95:c7:65:fa:57".to_string(), 188),
        ("f1:1d:cd:6a:88:a4".to_string(), 189),
        ("c6:b1:3d:58:4b:c3".to_string(), 190),
        ("d5:d6:9a:ef:a0:09".to_string(), 191),
        ("df:ad:5c:5e:7b:c1".to_string(), 192),
    ]);
    return *idmap.get(&mac.to_string()).unwrap();
}
