import paho.mqtt.client as mqtt
import threading

class BtSmartController:
    def __init__(self, broker="localhost", port=1883):
        self.l1 = 0xffff
        self.l2 = 0xffff
        self.l3 = 0xffff
        self.l4 = 0xffff

        self.client = mqtt.Client()

        # Assign the on_message callback function
        self.client.on_message = self.on_message

        # Connect to the MQTT broker
        self.client.connect(broker, port, 60)

        # Subscribe to the topic
        self.client.subscribe("bt_smart_controller/i1")
        self.client.subscribe("bt_smart_controller/i2")
        self.client.subscribe("bt_smart_controller/i3")
        self.client.subscribe("bt_smart_controller/i4")

        # Start a background thread to handle the network loop
        self.thread = threading.Thread(target=self.client.loop_forever)
        self.thread.daemon = True
        self.thread.start()

        self.write_m1(0)
        self.write_m2(0)

    def read_l1(self):
        return self.l1

    def read_l2(self):
        return self.l2

    def read_l3(self):
        return self.l3

    def read_l4(self):
        return self.l4

    def write_m1(self, val):
        self.client.publish("bt_smart_controller/m1", str(val).encode("utf-8"))

    def write_m2(self, val):
        self.client.publish("bt_smart_controller/m2", str(val).encode("utf-8"))

    def on_message(self, client, userdata, message):
        msg_payload = message.payload.decode('utf-8')
        match message.topic:
            case "bt_smart_controller/i1":
                self.l1 = int(msg_payload)
            case "bt_smart_controller/i2":
                self.l2 = int(msg_payload)
            case "bt_smart_controller/i3":
                self.l3 = int(msg_payload)
            case "bt_smart_controller/i4":
                self.l4 = int(msg_payload)


    def stop(self):
        self.client.loop_stop()
        self.thread.join()


