use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let cookie = read_cookie();
    println!("{}", cookie);
    let year = "2021";
    let day = "1";
    let input = get_input(year, day);
}

fn read_cookie() -> String {
    return fs::read_to_string("cookie.txt").expect("Failed to read cookie.txt");
}

fn get_input(year:&str, day:&str) -> String {
    let input_path = get_input_path(year, day);
    let result = String::from("temp");
    println!("{}", input_path.display());
    return result;
    /*fetch_input_from_aoc(year, day, inputPath);
    fs.readFile(inputPath, (err, data) => {
        if (err)
            fetchInput(year, day, inputPath)
        else
            runDay(year, day, data.toString())
    })*/
}

fn get_input_path(year:&str, day:&str) -> PathBuf {
    let mut path = env::current_dir().expect("Couldn't read current dir."); 
    path.push("input");
    let mut yearday = String::from(year);
    yearday.push_str("_");
    yearday.push_str(day);
    path.push(yearday);
    path.set_extension("txt");
    path
}

/*function fetch_input_from_aoc(year, day, inputPath) {
    const path = `/${year}/day/${day}/input`
    const options = {
        host: 'adventofcode.com',
        path: path,
        method: 'GET',
        headers: {
            'Cookie': cookie
        }
    }

    let req = https.request(options, res => {
        console.log(`statusCode: ${res.statusCode}`)
        let body = []
        res.on('data', (d) => {
            body.push(d)
        }).on('end', () => {
            let result = Buffer.concat(body).toString()
            fs.outputFile(inputPath, result).then(() => {
                runDay(year, day, result)
            }).catch(err => {
                console.error(err)
            })
        })
    })

    req.on('error', (error) => {
        console.error(error)
    })

    req.end()
}*/