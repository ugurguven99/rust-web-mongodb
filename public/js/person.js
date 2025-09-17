
//Person ID yi databaseden çekmede hata var belkide person_id yazıp denenmeli
async function fetchPersons() {
    const response = await fetch('/persons');
    const persons = await response.json();
    const tableBody = document.getElementById('personTable').querySelector('tbody');
    tableBody.innerHTML = '';
    persons.forEach(person => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${person.id ? person.id : ''}</td>
            <td>${person.isim}</td>
            <td>${person.soyisim}</td>
            <td>${person.email}</td>
            <td>${person.yas}</td>
            <td>
                <button class="btn btn-warning" onclick="editPerson(${person.id})">Edit</button>
                <button class="btn btn-danger" onclick="deletePerson(${person.id})">Delete</button>
            </td>
        `;
        tableBody.appendChild(row);
    });
}

async function deletePerson(personId) {
    $('#deleteModal').modal('show');
    document.getElementById('confirmDeleteButton').onclick = async function () {
        const response = await fetch(`/delete_person/${personId}`, {
            method: 'DELETE'
        });
        if (response.ok) {
            showModal('Kişi silme başarılı!', 'successModal');
            fetchPersons();
        } else {
            showModal('Kişi silme başarısız!', 'errorModal');
        }
        $('#deleteModal').modal('hide');
    }
}

function editPerson(personId) {
    const person = persons.find(p => p.id === personId);
    if (person) {
        document.getElementById('editPersonId').value = person.id;
        document.getElementById('editIsim').value = person.isim;
        document.getElementById('editSoyisim').value = person.soyisim;
        document.getElementById('editEmail').value = person.email;
        document.getElementById('editYas').value = person.yas;
        $('#editModal').modal('show');
    }
}

async function submitEditForm() {
    const personId = document.getElementById('editPersonId').value;
    const personData = {
        isim: document.getElementById('editIsim').value,
        soyisim: document.getElementById('editSoyisim').value,
        email: document.getElementById('editEmail').value,
        yas: document.getElementById('editYas').value
    };

    const response = await fetch(`/update_person/${personId}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(personData)
    });

    if (response.ok) {
        showModal('Kişi güncelleme başarılı!', 'successModal');
        $('#editModal').modal('hide');
        fetchPersons();
    } else {
        showModal('Kişi güncelleme başarısız!', 'errorModal');
    }
}
