use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

/// ###Формат описания:
///  name (pos): value - description, ...
///
///  Где:
///
///     name - имя переменной
///     pos - положение в строке
///     value - числовое, логическое значение или диапазон от a до b с шагом c (a..b/c)
///     description - описание значения
///
/// ###Строка 1:
///     name (0..80): Текстовая информация
/// ###Строка 2:
///
///     j0 (1..3): 1 - характерная площадь крыла задана, 0 - характерная площадь крыла не задана
///
///     j1 (4..6): 0 - крыло отсутствует, 1 - крыло имеет неплоскую  поверхность. При этом возможны два
///                 варианта задания неплоской серединной поверхности:
///                 * Профиль в каждом сечении по размаху крыла задан координатами средней линии
///                     и полутолщинами.
///                 * Профиль в каждом сечении по размаху крыла задан координатами верхней и нижней
///                     поверхности.
///                 В обоих случаях может быть задана крутка
///        -1 - Крыло плоское. Профиль в каждом сечении по размаху крыла задан координатами
///              полутолщины. В случае крыла нулевой толщины координаты полутолщин должны быть
///              заданы нулевыми.
///
///     j2 (7..9): 0 - фюзеляж отсутствует, 1 - фюзеляж имеет произвольные поперечные сечения,
///               -1 - фюзеляж имеет круговые поперечные сечения (если J6=0, фюзеляж имеет
///                    деформированную ось). Если J6 = -1, фюзеляж имеет симметрию
///                    относительно плоскости y=0. Если J6 = 1 – весь летательный аппарат симметричен
///                    относительно плоскости y=0.
///
///     j3 (10..12): 0 - подвесные грузы отсутствуют, 1 - подвесные грузы присутствуют
///
///     j4 (13..15): 0 - вертикальное оперение отсутствует, 1 - вертикальное оперение присутствует
///
///     j5 (16..18): 0 - горизонтальное оперение отсутствует, 1 - горизонтальное оперение присутствует
///
///     j6 (19..21): 0 - Фюзеляж имеет круговые сечения с деформированной осью или произвольные
///                      (в случае J2≠0)
///                  1 - Весь летательный аппарат симметричен относительно плоскости y=0.
///                 -1 - Фюзеляж имеет круговые поперечные сечения при J2≠0. Ось фюзеляжа не деформирована.
///
///     nwaf (22..24): 2..20 - Число сечений на полуразмахе крыла.
///
///     nwafor (25..27): 3..30 - Число точек, используемых для описания профиля в каждом сечении крыла.
///                              Если NWAFOR вводится с отрицательным знаком, программа вводит координаты нижней
///                              и верхней поверхности крыла.
///
///     nfus (28..30): 1..4 - Число сегментов, на которые разбит фюзеляж.
///
///     nradx_<n> (31..51/6): 3..30 - Число точек, используемых для описания полусечения n сегмента
///                                   фюзеляжа. Если фюзеляж представляет собой тело вращения, программа сама
///                                   определяет указанное количество координат Y и Z.
///
///     nforx_<n> (31..54/6): 2..30 - Число сечений на n сегменте фюзеляжа.
///
///     np (55..57): 0..9 - число подвесок
///
///     kfield (58..60): 0 - Расчёт полей течения отсутствует, 1 - Присутствует расчёт полей течения.
///
///     nf (61..63): 0..6 - Число вертикальных оперений.
///
///     nfinor (64..66): 3..30 - Число точек, в которых заданы координаты профиля для каждого
///                              вертикального оперения.
///
///     ncan (67..69): 0..6 - Число горизонтальных оперений.
///
///     ncanor (70..72): 3..30 - Число точек, в которых заданы координаты профиля для каждого горизонтального
///                              оперения. Если NCANOR отрицательная величина, необходимо задавать координаты
///                              верхней и нижней поверхности профиля. Если NCANOR положительная величина,
///                              рассматривается только симметричный профиль на горизонтальном оперении.
///
/// ###Строка 3:
///
///     Содержит логические переменные
///
///     itemax (1..5): true - Решается задача обтекания изолированного фюзеляжа итерационным
///                           методом, когда нелинейные эффекты трансзвукового потока учитываются
///                           с помощью местных чисел Маха.
///                           Примечание:
///                                 Если itemax=true, задача обтекания решается только для
///                                 изолированного фюзеляжа, скоростного поезда, автомобиля.
///                                 В противном случае программа выдаст ошибку с соответствующей
///                                 диагностикой (при наличии на фюзеляже, скоростном поезде,
///                                 автомобиле несущих элементов)
///                     false - Сформулированная выше задача не решается.
///
///     ground (8..12): true - Учёт влияния земли для изолированного фюзеляжа, скоростного поезда,
///                             автомобиля.
///                     false - Нет учёта влияния земли для изолированного фюзеляжа, скоростного
///                             поезда, автомобиля.
///
///     diver (22..26): true - Расчёт аэродинамических характеристик компоновки (подготовка файлов)
///                            для расчёта на статическую прочность. Итерационный процесс отсутствует.
///                     false - Подготовка выходных файлов из программы «AEFLOT» для статической
///                             прочности отсутствует.
///
///     beloyc (29..31): true - Расчёт аэродинамических характеристик компоновки с учётом
///                             статической прочности (присутствует итерационный процесс).
///                      false - Выдача файлов из программы статической прочности для «AEFLOT»
///                              отсутствует.
///     shek (36..40): true - Выдаются компоненты скоростей на несущих поверхностях.
///                           Такая информация необходима для расчёта вязких течений.
///                    false - Компоненты скоростей на несущих поверхностях не выдаются.
///
/// ###Строка 3_:
///     Присутствует, если itemax == true
///
///     В формате I3 задается число итераций, которые необходимы для решения задачи с учётом
///     нелинейных эффектов (местных чисел Маха).
///
/// ###Строка 4:
///     Присутствуют, если J3 != 0. Задание исходных данных подвесных грузов. (9 значений npodor)
///
///     Если NPODOR == 0, поперечные сечения 1-ой подвески круговые. Ось подвески не деформирована.
///     Если NPODOR == 1, поперечные сечения 1-ой подвески круговые. Ось подвески деформирована.
///     Если NPODOR == -1, поперечные сечения 1-й подвески произвольные.
///
/// ###Строка 5:
///     Присутствует, если J3 != 0. Задание исходнахданных подвесных грузов.
///     9 значений npradx чередующиеся с 9 значениями npusor
///
///     npradx[n] - Число точек, используемых для описания полусечения n-й подвески.
///     npusor[n] - Число поперечных сечений на n-й подвеске.
///
///
/// ###Строка 6:
///     Присутствует, если kfield == 1.
///     kxf (1..3): Число точек поля (в направлении оси X), в которых вычисляются компоненты
///                 возмущенных скоростей. Максимальное число точек – 30.
///
///     kyf (4..6): Число точек поля (в направлении оси Z), в которых вычисляются компоненты
///                 возмущённых скоростей, для каждого фиксированного значения X.
///                 Максимальное число KYF равно 20.
///
///                     Замечание: В программе число точек в направлении оси Y равно KYF.
///
/// ###Строка 7:
///     Значение характерной площади крыла задается в позициях с 1-ой по 7-ю. Эта строка необходима
///     только в случае, если в строке 2  J0=1.
///     значение в wing_area
///
/// ###Строки 8..:
///     wing_data
///     Начиная со строки 8, следуют исходные данные крыла (только в случае,
///     если в строке 2  J1 != 0). Первая строка (или строки) крыла содержат NWAFOR координат
///     абсцисс профиля крыла, выраженных в процентах местной хорды.
///     Следующая группа строк (NWAF-строк), относящихся к крылу, содержат по четыре числа:
///         координаты x,z,y,b передней кромки и длин хорд каждого профиля. Порядок следования строк
///         должен соответствовать номерам сечений, начиная от бортового и заканчивая концевым
///         (если фюзеляж отсутствует, тогда порядок следования строк должен соответствовать
///         номерам сечений, начиная от корневого и заканчивая концевым).
///         x (1..7): координата передней кромки профиля в сечении по размаху крыла.
///         z (8..14): координата передней кромки профиля в сечении по размаху крыла.
///         y (15..21): координата передней кромки профиля в сечении по размаху крыла.
///         b (22..28): длина хорды профиля в сечении по размаху крыла.
#[derive(Debug)]
pub struct AeflotInput {
    //1 строка
    pub name: String,
    //2 строка
    pub j0: i8, pub j1: i8, pub j2: i8, pub j3: i8, pub j4: i8, pub j5: i8, pub j6: i8,
    pub nwaf: isize, pub nwafor: isize, pub nfus: i8,
    pub nradx_1: isize, pub nforx_1: isize, pub nradx_2: isize, pub nforx_2: isize,
    pub nradx_3: isize, pub nforx_3: isize, pub nradx_4: isize, pub nforx_4: isize,
    pub np: i8, pub kfield: i8, pub nf: i8, pub nfinor: isize, pub ncan: i8, pub ncanor: isize,
    //3 строка
    pub itemax: bool, pub ground: bool, pub bet: bool,
    pub diver: bool, pub beloyc: bool, pub shek: bool,
    //3_1 строка
    //4 строка
    pub npodor: [i8; 9],
    //5 строка
    pub npradx: [isize; 9],
    pub npusor: [isize; 9],
    //6 строка
    pub kxf: i8, pub kyf: i8,
    //7 строка
    pub wing_area: f64,
    //8.. строка и, возможно, больше
    pub wing_coord_percent: Vec<f64>,
    //следующая группа nwaf строк
    pub wing_data: Vec<[f64; 4]>

}

impl Default for AeflotInput { fn default() -> Self { Self::new() } }

impl AeflotInput {
    pub fn new() -> AeflotInput {
        AeflotInput {
            name: String::new(),
            j0: 0,
            j1: 0,
            j2: 0,
            j3: 0,
            j4: 0,
            j5: 0,
            j6: 0,
            nwaf: 2,
            nwafor: 3,
            nfus: 1,
            nradx_1: 3,
            nforx_1: 2,
            nradx_2: 3,
            nforx_2: 2,
            nradx_3: 3,
            nforx_3: 2,
            nradx_4: 3,
            nforx_4: 2,
            np: 0,
            kfield: 0,
            nf: 0,
            nfinor: 3,
            ncan: 0,
            ncanor: 3,
            itemax: false,
            ground: false,
            bet: false,
            diver: false,
            beloyc: false,
            shek: false,
            npodor: [0; 9],
            npradx: [3; 9],
            npusor: [2; 9],
            kxf: 0,
            kyf: 0,
            wing_area: 0.0,
            wing_coord_percent: vec![],
            wing_data: vec![]
        }
    }

    pub fn read(path: &str) -> AeflotInput {
        let file = File::open(path).unwrap();
        let mut aeflot_file = AeflotInput::new();
        aeflot_file.parse(file);
        aeflot_file
    }

    fn parse(&mut self, file: File) {
        let mut file_iterator = BufReader::new(file).lines();
        self.name = String::from(file_iterator.next().unwrap().unwrap().trim());
        self.parse_2_line(file_iterator.next().unwrap().unwrap());
        self.parse_3_line(file_iterator.next().unwrap().unwrap());
        if self.itemax {
            self.parse_3_1_line(file_iterator.next().unwrap().unwrap())
        };
        if self.j3 != 0 {
            self.parse_4_line(file_iterator.next().unwrap().unwrap());
            self.parse_5_line(file_iterator.next().unwrap().unwrap());
        };
        if self.kfield == 1 {
            self.parse_6_line(file_iterator.next().unwrap().unwrap());
        }
        if self.j0 == 1 {
            self.wing_area = str_to_f64(&mut get_substring(
                &file_iterator.next().unwrap().unwrap(), 0, 6)
            );
        }
        { // блок извлечения nwafor координат из ??? строк
            let nwafor = self.nwafor.abs();
            self.wing_coord_percent.reserve(nwafor as usize);
            //эмпирически одна координата 7 позиций
            let req_len = 7;
            // nwafor * req_len - общее количество позиций для nwafor координат
            // num_of_lines - количество строк, требуемых для записи nwafor координат
            let num_of_lines = ((nwafor * req_len) as f64 / 80.0).ceil() as isize;
            let req_len: usize = req_len as usize;
            for _ in 0..num_of_lines {
                let line = file_iterator.next().unwrap().unwrap();
                for (start, end) in (req_len..line.len() + req_len)
                    .step_by(req_len).enumerate() {
                    self.wing_coord_percent.push(
                        str_to_f64(
                            &mut get_substring(&line, start * req_len, end)
                        )
                    );
                }
            }
        }
        self.wing_data.reserve(self.nwaf.abs() as usize);
        for _ in 0..self.nwaf {
            self.parse_nwaf_line(file_iterator.next().unwrap().unwrap())
        }
    }
}

//Парсеры для частных случаев
impl AeflotInput {
    fn parse_2_line(&mut self, line: String) {
        let mut value = String::with_capacity(10);
        for (char_no, char) in line.chars().enumerate() {
            value.push(char);
            match char_no {
                //смещение всех номеров позиций влево на 1 из-за нумерации с 0
                2 => { self.j0 = str_to_i8(&mut value) },
                5 => { self.j1 = str_to_i8(&mut value) },
                8 => { self.j2 = str_to_i8(&mut value) },
                11 => { self.j3 = str_to_i8(&mut value) },
                14 => { self.j4 = str_to_i8(&mut value) },
                17 => { self.j5 = str_to_i8(&mut value) },
                20 => { self.j6 = str_to_i8(&mut value) },
                23 => { self.nwaf = str_to_isize(&mut value) },
                26 => { self.nwafor = str_to_isize(&mut value) },
                29 => { self.nfus = str_to_i8(&mut value) },
                32 => { self.nradx_1 = str_to_isize(&mut value) },
                35 => { self.nforx_1 = str_to_isize(&mut value) },
                38 => { self.nradx_2 = str_to_isize(&mut value) },
                41 => { self.nforx_2 = str_to_isize(&mut value) },
                44 => { self.nradx_3 = str_to_isize(&mut value) },
                47 => { self.nforx_3 = str_to_isize(&mut value) },
                50 => { self.nradx_4 = str_to_isize(&mut value) },
                53 => { self.nforx_4 = str_to_isize(&mut value) },
                56 => { self.np = str_to_i8(&mut value) },
                59 => { self.kfield = str_to_i8(&mut value) },
                65 => { self.nfinor = str_to_isize(&mut value) },
                62 => { self.nf = str_to_i8(&mut value) },
                68 => { self.ncan = str_to_i8(&mut value) },
                71 => { self.ncanor = str_to_isize(&mut value) },
                _ => { continue }
            }
        }
    }

    fn parse_3_line(&mut self, line: String) {
        let mut value = String::with_capacity(10);
        for (char_no, char) in line.chars().enumerate() {
            value.push(char);
            match char_no {
                //смещение всех номеров позиций влево на 1 из-за нумерации с 0
                6 => { self.itemax = str_to_bool(&mut value) },
                13 => { self.ground = str_to_bool(&mut value) },
                20 => { self.bet = str_to_bool(&mut value) },
                27 => { self.diver = str_to_bool(&mut value) },
                34 => { self.beloyc = str_to_bool(&mut value) },
                39 => { self.shek = str_to_bool(&mut value) },
                _ => { continue }
            }
        }
    }

    fn parse_3_1_line(&mut self, line: String) {
        todo!()
    }

    fn parse_4_line(&mut self, line: String) {
        for (start, end) in (3..line.len()).step_by(3).enumerate() {
            if start > 8 { continue };
            self.npodor[start] = str_to_i8(&mut get_substring(&line, start * 3, end))
        }
    }

    fn parse_5_line(&mut self, line: String) {
        for (start, end) in (3..line.len()).step_by(3).enumerate() {
            if start / 2 > 9 { continue };
            if start % 2 == 0 {
                self.npusor[start / 2] = str_to_isize(
                    &mut get_substring(&line, (start / 2) * 3, end)
                )
            }
            else {
                self.npradx[start] = str_to_isize(
                    &mut get_substring(&line, start * 3, end)
                )
            }
        }
    }

    fn parse_6_line(&mut self, line: String) {
        self.kxf = str_to_i8(&mut get_substring(&line, 0, 2));
        self.kyf = str_to_i8(&mut get_substring(&line, 3, 5));
    }

    fn parse_nwaf_line(&mut self, line: String) {
        let mut coords: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        coords[0] = str_to_f64(&mut get_substring(&line, 0, 6));
        coords[1] = str_to_f64(&mut get_substring(&line, 7, 13));
        coords[2] = str_to_f64(&mut get_substring(&line, 14, 20));
        coords[3] = str_to_f64(&mut get_substring(&line, 21, 27));
        self.wing_data.push(coords)
    }
}

fn str_to_i8(value: &mut String) -> i8 {
    let number_value = i8::from_str_radix(value.trim(), 10).unwrap();
    value.clear();
    number_value
}

fn str_to_isize(value: &mut String) -> isize {
    let number_value = isize::from_str_radix(value.trim(), 10).unwrap();
    value.clear();
    number_value
}

fn str_to_bool(value: &mut String) -> bool {
    let out = value.trim().to_uppercase() == "TRUE";
    value.clear();
    out
}

fn str_to_f64(value: &mut String) -> f64 {
    let out = f64::from_str(value.trim()).unwrap();
    value.clear();
    out
}

fn get_substring(string: &String, start: usize, end: usize) -> String {
    let slice: String = string.chars()
        .into_iter()
        .skip(start)
        .take(end - start)
        .collect();
    slice
}