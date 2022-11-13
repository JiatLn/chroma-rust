use std::collections::HashMap;

lazy_static! {
    /// X11 color names
    ///
    /// http://www.w3.org/TR/css3-color/#svg-color
    pub static ref W3CX11_HASHMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("aliceblue", "#f0f8ff");
        m.insert("antiquewhite", "#faebd7");
        m.insert("aqua", "#00ffff");
        m.insert("aquamarine", "#7fffd4");
        m.insert("azure", "#f0ffff");
        m.insert("beige", "#f5f5dc");
        m.insert("bisque", "#ffe4c4");
        m.insert("black", "#000000");
        m.insert("blanchedalmond", "#ffebcd");
        m.insert("blue", "#0000ff");
        m.insert("blueviolet", "#8a2be2");
        m.insert("brown", "#a52a2a");
        m.insert("burlywood", "#deb887");
        m.insert("cadetblue", "#5f9ea0");
        m.insert("chartreuse", "#7fff00");
        m.insert("chocolate", "#d2691e");
        m.insert("coral", "#ff7f50");
        m.insert("cornflowerblue", "#6495ed");
        m.insert("cornsilk", "#fff8dc");
        m.insert("crimson", "#dc143c");
        m.insert("cyan", "#00ffff");
        m.insert("darkblue", "#00008b");
        m.insert("darkcyan", "#008b8b");
        m.insert("darkgoldenrod", "#b8860b");
        m.insert("darkgray", "#a9a9a9");
        m.insert("darkgreen", "#006400");
        m.insert("darkgrey", "#a9a9a9");
        m.insert("darkkhaki", "#bdb76b");
        m.insert("darkmagenta", "#8b008b");
        m.insert("darkolivegreen", "#556b2f");
        m.insert("darkorange", "#ff8c00");
        m.insert("darkorchid", "#9932cc");
        m.insert("darkred", "#8b0000");
        m.insert("darksalmon", "#e9967a");
        m.insert("darkseagreen", "#8fbc8f");
        m.insert("darkslateblue", "#483d8b");
        m.insert("darkslategray", "#2f4f4f");
        m.insert("darkslategrey", "#2f4f4f");
        m.insert("darkturquoise", "#00ced1");
        m.insert("darkviolet", "#9400d3");
        m.insert("deeppink", "#ff1493");
        m.insert("deepskyblue", "#00bfff");
        m.insert("dimgray", "#696969");
        m.insert("dimgrey", "#696969");
        m.insert("dodgerblue", "#1e90ff");
        m.insert("firebrick", "#b22222");
        m.insert("floralwhite", "#fffaf0");
        m.insert("forestgreen", "#228b22");
        m.insert("fuchsia", "#ff00ff");
        m.insert("gainsboro", "#dcdcdc");
        m.insert("ghostwhite", "#f8f8ff");
        m.insert("gold", "#ffd700");
        m.insert("goldenrod", "#daa520");
        m.insert("gray", "#808080");
        m.insert("green", "#008000");
        m.insert("greenyellow", "#adff2f");
        m.insert("grey", "#808080");
        m.insert("honeydew", "#f0fff0");
        m.insert("hotpink", "#ff69b4");
        m.insert("indianred", "#cd5c5c");
        m.insert("indigo", "#4b0082");
        m.insert("ivory", "#fffff0");
        m.insert("khaki", "#f0e68c");
        m.insert("laserlemon", "#ffff54");
        m.insert("lavender", "#e6e6fa");
        m.insert("lavenderblush", "#fff0f5");
        m.insert("lawngreen", "#7cfc00");
        m.insert("lemonchiffon", "#fffacd");
        m.insert("lightblue", "#add8e6");
        m.insert("lightcoral", "#f08080");
        m.insert("lightcyan", "#e0ffff");
        m.insert("lightgoldenrod", "#fafad2");
        m.insert("lightgoldenrodyellow", "#fafad2");
        m.insert("lightgray", "#d3d3d3");
        m.insert("lightgreen", "#90ee90");
        m.insert("lightgrey", "#d3d3d3");
        m.insert("lightpink", "#ffb6c1");
        m.insert("lightsalmon", "#ffa07a");
        m.insert("lightseagreen", "#20b2aa");
        m.insert("lightskyblue", "#87cefa");
        m.insert("lightslategray", "#778899");
        m.insert("lightslategrey", "#778899");
        m.insert("lightsteelblue", "#b0c4de");
        m.insert("lightyellow", "#ffffe0");
        m.insert("lime", "#00ff00");
        m.insert("limegreen", "#32cd32");
        m.insert("linen", "#faf0e6");
        m.insert("magenta", "#ff00ff");
        m.insert("maroon", "#800000");
        m.insert("maroon2", "#7f0000");
        m.insert("maroon3", "#b03060");
        m.insert("mediumaquamarine", "#66cdaa");
        m.insert("mediumblue", "#0000cd");
        m.insert("mediumorchid", "#ba55d3");
        m.insert("mediumpurple", "#9370db");
        m.insert("mediumseagreen", "#3cb371");
        m.insert("mediumslateblue", "#7b68ee");
        m.insert("mediumspringgreen", "#00fa9a");
        m.insert("mediumturquoise", "#48d1cc");
        m.insert("mediumvioletred", "#c71585");
        m.insert("midnightblue", "#191970");
        m.insert("mintcream", "#f5fffa");
        m.insert("mistyrose", "#ffe4e1");
        m.insert("moccasin", "#ffe4b5");
        m.insert("navajowhite", "#ffdead");
        m.insert("navy", "#000080");
        m.insert("oldlace", "#fdf5e6");
        m.insert("olive", "#808000");
        m.insert("olivedrab", "#6b8e23");
        m.insert("orange", "#ffa500");
        m.insert("orangered", "#ff4500");
        m.insert("orchid", "#da70d6");
        m.insert("palegoldenrod", "#eee8aa");
        m.insert("palegreen", "#98fb98");
        m.insert("paleturquoise", "#afeeee");
        m.insert("palevioletred", "#db7093");
        m.insert("papayawhip", "#ffefd5");
        m.insert("peachpuff", "#ffdab9");
        m.insert("peru", "#cd853f");
        m.insert("pink", "#ffc0cb");
        m.insert("plum", "#dda0dd");
        m.insert("powderblue", "#b0e0e6");
        m.insert("purple", "#800080");
        m.insert("purple2", "#7f007f");
        m.insert("purple3", "#a020f0");
        m.insert("rebeccapurple", "#663399");
        m.insert("red", "#ff0000");
        m.insert("rosybrown", "#bc8f8f");
        m.insert("royalblue", "#4169e1");
        m.insert("saddlebrown", "#8b4513");
        m.insert("salmon", "#fa8072");
        m.insert("sandybrown", "#f4a460");
        m.insert("seagreen", "#2e8b57");
        m.insert("seashell", "#fff5ee");
        m.insert("sienna", "#a0522d");
        m.insert("silver", "#c0c0c0");
        m.insert("skyblue", "#87ceeb");
        m.insert("slateblue", "#6a5acd");
        m.insert("slategray", "#708090");
        m.insert("slategrey", "#708090");
        m.insert("snow", "#fffafa");
        m.insert("springgreen", "#00ff7f");
        m.insert("steelblue", "#4682b4");
        m.insert("tan", "#d2b48c");
        m.insert("teal", "#008080");
        m.insert("thistle", "#d8bfd8");
        m.insert("tomato", "#ff6347");
        m.insert("turquoise", "#40e0d0");
        m.insert("violet", "#ee82ee");
        m.insert("wheat", "#f5deb3");
        m.insert("white", "#ffffff");
        m.insert("whitesmoke", "#f5f5f5");
        m.insert("yellow", "#ffff00");
        m.insert("yellowgreen", "#9acd32");
        m
    };
}