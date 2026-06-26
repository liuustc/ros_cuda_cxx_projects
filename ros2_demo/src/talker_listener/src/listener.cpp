#include <memory>
#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/string.hpp"

class Listener : public rclcpp::Node
{
public:
  Listener() : Node("listener")
  {
    rclcpp::QoS qos(rclcpp::KeepLast(10));
    qos.reliable();
    publisher_ = this->create_publisher<std_msgs::msg::String>("reply", qos);
    subscription_ = this->create_subscription<std_msgs::msg::String>(
      "chatter", qos,
      [this](const std_msgs::msg::String::SharedPtr msg) {
        RCLCPP_INFO(this->get_logger(), "[Listener] 收到: '%s'", msg->data.c_str());

        // 回复消息
        auto reply = std_msgs::msg::String();
        reply.data = "已收到! -> " + msg->data;
        RCLCPP_INFO(this->get_logger(), "[Listener] 回复: '%s'", reply.data.c_str());
        publisher_->publish(reply);
      });
  }

private:
  rclcpp::Publisher<std_msgs::msg::String>::SharedPtr publisher_;
  rclcpp::Subscription<std_msgs::msg::String>::SharedPtr subscription_;
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<Listener>());
  rclcpp::shutdown();
  return 0;
}
