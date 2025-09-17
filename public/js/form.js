//Form gönderme fonksiyonu
async function submitForm() {
    const form = document.getElementById('personForm');
    const formData = new FormData(form);
    const data = Object.fromEntries(formData.entries());

    const response = await fetch('/add_person', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    if (response.ok) {
        showModal('Kişi ekleme başarılı!', 'successModal');
        form.reset();
        fetchPersons();
    } else {
        showModal('Kişi ekleme başarısız!', 'errorModal');
    }
}
