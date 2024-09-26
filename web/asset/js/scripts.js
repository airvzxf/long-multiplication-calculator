class Warning {
    static class_hidden = 'hidden'
    static error_message = document.getElementById('error-message')

    static show() {
        if (this.error_message.classList.contains(this.class_hidden)) {
            this.error_message.classList.remove(this.class_hidden);
        }
    }

    static hide() {
        if (!this.error_message.classList.contains(this.class_hidden)) {
            this.error_message.classList.add(this.class_hidden);
        }
    }
}

function warning_clean_up() {
    const solution = document.getElementById('solution')
    solution.innerText = ""
}

function calculate() {
    Warning.hide()
    const multiplier_element = document.getElementById('multiplier')
    const multiplicand_element = document.getElementById('multiplicand')
    const solution = document.getElementById('solution')
    const error_text = document.getElementById('error-text')
    solution.innerText = "Calculating multiplication…"

    const multiplier = parseInt(multiplier_element.value)
    const multiplicand = parseInt(multiplicand_element.value)

    if (isNaN(multiplier)) {
        error_text.innerText = "The multiplier is required. Please, fill the field."
        Warning.show()
        warning_clean_up()
        multiplier_element.focus()
        return
    }

    if (isNaN(multiplicand)) {
        error_text.innerText = "The multiplicand is required. Please, fill the field."
        Warning.show()
        warning_clean_up()
        multiplicand_element.focus()
        return
    }

    const url = "asset/bash/init.bash?" +
        multiplier_element.value + "," +
        multiplicand_element.value

    let xml_http_request = new XMLHttpRequest()
    xml_http_request.open("GET", url, true)
    xml_http_request.setRequestHeader('Content-type', 'application/x-www-form-urlencoded')
    xml_http_request.onreadystatechange = function () {
        if (this.readyState === 4 && this.status === 200) {
            solution.innerText = this.responseText
        } else if (this.status !== 200) {
            error_text.innerHTML = "Fatal Error in the Ajax request. Failed trying to call the `asset/bash/init.bash`."
            error_text.innerHTML += "<br>"
            error_text.innerHTML += "<br>"
            error_text.innerHTML += "<strong>Status:</strong> " + this.status
            error_text.innerHTML += "<br>"
            error_text.innerHTML += "<strong>Response:</strong><br>"
            error_text.innerHTML += this.responseText
            Warning.show()
            warning_clean_up()
        }
    }
    xml_http_request.send()
}

{
    const multiplier_element = document.getElementById('multiplier')
    const multiplicand_element = document.getElementById('multiplicand')

    multiplier_element.addEventListener("keypress", (element_event) => {
        if (element_event.key === "Enter") {
            calculate()
        }
    });

    multiplicand_element.addEventListener("keypress", (element_event) => {
        if (element_event.key === "Enter") {
            calculate()
        }
    });
}
