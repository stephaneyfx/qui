import QtQuick 2.6
import QtQuick.Controls 2.2
import QtQuick.Controls.Material 2.2
import QtQuick.Layouts 1.1
import QtQuick.Window 2.2

Pane {
    width: 400
    height: 400
    Material.theme: Material.Dark

    ColumnLayout {
        spacing: 5

        Switch {}

        TextField {
            Layout.fillWidth: true
            placeholderText: "Language"
        }
    }
}
