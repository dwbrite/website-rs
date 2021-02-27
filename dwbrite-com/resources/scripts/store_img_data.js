function store_img_data(name, data) {
    let dict = localStorage.getItem("mediaDict");
    if (dict == null) {
        dict = {};
    } else {
        dict = JSON.parse(dict);
    }

    dict[name] = data;
    localStorage.setItem("mediaDict", JSON.stringify(dict));
}