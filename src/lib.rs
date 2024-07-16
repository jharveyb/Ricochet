use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (x, y, board_size=16))]
fn idx(x: u8, y: u8, board_size: Option<u8>) -> PyResult<u8> {
    // Default to a full 16x16 board.
    let size = board_size.unwrap_or(16);

    // This could technically be larger than a u8, but in practice the max value
    // for size is 16, and 15 for x and y.
    let index = x + (y * size) as u8;

    return Ok(index);
}

#[pyfunction]
#[pyo3(signature = (index, board_size=16))]
fn xy(index: u8, board_size: Option<u8>) -> PyResult<(u8, u8)> {
    // Default to a full 16x16 board.
    let size = board_size.unwrap_or(16);

    // Index 0 is the top left corner, not bottom left.
    let x = index % size;
    let y = index / size;

    return Ok((x, y));
}

#[pymodule]
fn rroboter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(xy, m)?)?;
    m.add_function(wrap_pyfunction!(idx, m)?)?;
    Ok(())
}
