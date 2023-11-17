// 定义表示交通信号灯的枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个包含获取每个灯持续时间的方法的 trait
trait Duration {
    fn get_duration(&self) -> u32;
}

// 为 TrafficLight 枚举实现 Duration trait
impl Duration for TrafficLight {
    fn get_duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

// 示例用法
fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("红灯持续时间：{} 秒", red_light.get_duration());
    println!("黄灯持续时间：{} 秒", yellow_light.get_duration());
    println!("绿灯持续时间：{} 秒", green_light.get_duration());
}
